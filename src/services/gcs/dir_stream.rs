// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Result;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use anyhow::anyhow;
use bytes::Buf;
use futures::{future::BoxFuture, ready, Future, Stream};
use isahc::AsyncReadResponseExt;
use serde::Deserialize;
use serde_json::de;

use crate::error::{other, ObjectError};
use crate::http_util::parse_error_response;
use crate::services::gcs::backend::Backend;
use crate::services::gcs::error::parse_error;
use crate::{DirEntry, ObjectMode};

enum State {
    Standby,
    Pending(BoxFuture<'static, Result<Vec<u8>>>),
    Walking((Output, usize, usize)),
}

/// DirStream takes over task of listing objects and
/// helps walking directory
pub struct DirStream {
    backend: Arc<Backend>,
    path: String,
    page_token: String,

    done: bool,
    state: State,
}

impl DirStream {
    /// Generate a new directory walker
    pub fn new(backend: Arc<Backend>, path: &str) -> Self {
        Self {
            backend,
            path: path.to_string(),
            page_token: "".to_string(), // don't know if it works?

            done: false,
            state: State::Standby,
        }
    }
}

impl Stream for DirStream {
    type Item = Result<DirEntry>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let backend = self.backend.clone();

        match &mut self.state {
            State::Standby => {
                let path = self.path.clone();
                let token = self.page_token.clone();

                let fut = async move {
                    let mut resp = backend.list_objects(&path, token.as_str()).await?;

                    if !resp.status().is_success() {
                        log::error!("GCS failed to list objects, status code: {}", resp.status());
                        let er = parse_error_response(resp).await?;
                        let err = parse_error("list", &path, er);
                        return Err(err);
                    }
                    let bytes = resp.bytes().await.map_err(|e| {
                        other(ObjectError::new(
                            "list",
                            &path,
                            anyhow!("read body: {:?}", e),
                        ))
                    })?;

                    Ok(bytes)
                };

                self.state = State::Pending(Box::pin(fut));
                self.poll_next(cx)
            }
            State::Pending(fut) => {
                let bytes = ready!(Pin::new(fut).poll(cx))?;
                let output: Output = de::from_reader(bytes.reader()).map_err(|e| {
                    other(ObjectError::new(
                        "list",
                        &self.path,
                        anyhow!("deserialize list_bucket output: {:?}", e),
                    ))
                })?;

                if let Some(token) = &output.next_page_token {
                    self.page_token = token.clone();
                } else {
                    self.done = true;
                }
                self.state = State::Walking((output, 0, 0));
                self.poll_next(cx)
            }
            State::Walking((output, common_prefixes_idx, objects_idx)) => {
                let prefixes = &output.prefixes;
                if *common_prefixes_idx < prefixes.len() {
                    let prefix = &prefixes[*common_prefixes_idx];
                    *common_prefixes_idx += 1;

                    let de = DirEntry::new(
                        backend.clone(),
                        ObjectMode::DIR,
                        &backend.get_rel_path(prefix),
                    );

                    log::debug!(
                        "object {} got entry, mode: {}, path: {}",
                        &self.path,
                        de.path(),
                        de.mode()
                    );
                    return Poll::Ready(Some(Ok(de)));
                }
                let objects = &output.items;
                while *objects_idx < objects.len() {
                    let object = &objects[*objects_idx];
                    *objects_idx += 1;

                    if object.name.ends_with('/') {
                        continue;
                    }

                    let de = DirEntry::new(
                        backend.clone(),
                        ObjectMode::FILE,
                        &backend.get_rel_path(&object.name),
                    );

                    log::debug!(
                        "dir object {} got entry, mode: {}, path: {}",
                        &self.path,
                        de.mode(),
                        de.path()
                    );
                    return Poll::Ready(Some(Ok(de)));
                }

                // end of asynchronous iteration
                if self.done {
                    log::debug!("object {} list done", &self.path);
                    return Poll::Ready(None);
                }

                self.state = State::Standby;
                self.poll_next(cx)
            }
        }
    }
}

/// Response JSON from GCS list objects API.
///
/// refer to https://cloud.google.com/storage/docs/json_api/v1/objects/list for details
#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Output {
    /// The kind of item this is.
    /// For lists of objects, this is always "storage#objects"
    kind: String,
    /// The continuation token.
    ///
    /// If this is the last page of results, then no continuation token is returned.
    next_page_token: Option<String>,
    /// Object name prefixes for objects that matched the listing request
    /// but were excluded from [items] because of a delimiter.
    prefixes: Vec<String>,
    /// The list of objects, ordered lexicographically by name.
    items: Vec<OutputContent>,
}

#[derive(Default, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OutputContent {
    name: String,
    size: String,
}

#[cfg(test)]
mod ds_test {
    use crate::services::gcs::dir_stream::Output;

    #[test]
    fn test_de_list() {
        let names = vec!["1.png", "2.png"];
        let sizes = vec![56535, 45506];

        let list_continuous = serde_json::de::from_str::<Output>(LIST_CONTINUOUS);
        assert!(list_continuous.is_ok());
        let list_continuous = list_continuous.unwrap();
        assert_eq!(
            list_continuous.next_page_token,
            Some("CgYxMC5wbmc=".to_string())
        );
        assert_eq!(list_continuous.items.len(), 2);
        for (idx, item) in list_continuous.items.iter().enumerate() {
            assert_eq!(item.name, names[idx]);
            let size_res = item.size.as_str().parse();
            assert!(size_res.is_ok());
            let size: u64 = size_res.unwrap();
            assert_eq!(size, sizes[idx]);
        }

        let list_discrete = serde_json::de::from_str::<Output>(LIST_DISCRETE);
        assert!(list_discrete.is_ok());
        let list_discrete = list_discrete.unwrap();
        assert!(list_discrete.next_page_token.is_none());
        assert_eq!(list_discrete.items.len(), 2);

        for (idx, item) in list_discrete.items.iter().enumerate() {
            assert_eq!(item.name, names[idx]);
            let size_res = item.size.as_str().parse();
            assert!(size_res.is_ok());
            let size: u64 = size_res.unwrap();
            assert_eq!(size, sizes[idx]);
        }
    }

    const LIST_DISCRETE: &str = r#"
    {
  "kind": "storage#objects",
  "prefixes": [
    "dir/",
    "test/"
  ],
  "items": [
    {
      "kind": "storage#object",
      "id": "example/1.png/1660563214863653",
      "selfLink": "https://www.googleapis.com/storage/v1/b/example/o/1.png",
      "mediaLink": "https://content-storage.googleapis.com/download/storage/v1/b/example/o/1.png?generation=1660563214863653&alt=media",
      "name": "1.png",
      "bucket": "example",
      "generation": "1660563214863653",
      "metageneration": "1",
      "contentType": "image/png",
      "storageClass": "STANDARD",
      "size": "56535",
      "md5Hash": "fHcEH1vPwA6eTPqxuasXcg==",
      "crc32c": "j/un9g==",
      "etag": "CKWasoTgyPkCEAE=",
      "timeCreated": "2022-08-15T11:33:34.866Z",
      "updated": "2022-08-15T11:33:34.866Z",
      "timeStorageClassUpdated": "2022-08-15T11:33:34.866Z"
    },
    {
      "kind": "storage#object",
      "id": "example/2.png/1660563214883337",
      "selfLink": "https://www.googleapis.com/storage/v1/b/example/o/2.png",
      "mediaLink": "https://content-storage.googleapis.com/download/storage/v1/b/example/o/2.png?generation=1660563214883337&alt=media",
      "name": "2.png",
      "bucket": "example",
      "generation": "1660563214883337",
      "metageneration": "1",
      "contentType": "image/png",
      "storageClass": "STANDARD",
      "size": "45506",
      "md5Hash": "e6LsGusU7pFJZk+114NV1g==",
      "crc32c": "L00QAg==",
      "etag": "CIm0s4TgyPkCEAE=",
      "timeCreated": "2022-08-15T11:33:34.886Z",
      "updated": "2022-08-15T11:33:34.886Z",
      "timeStorageClassUpdated": "2022-08-15T11:33:34.886Z"
    }
  ]
}
    "#;
    const LIST_CONTINUOUS: &str = r#"
    {
  "kind": "storage#objects",
  "prefixes": [
    "dir/",
    "test/"
  ],
  "nextPageToken": "CgYxMC5wbmc=",
  "items": [
    {
      "kind": "storage#object",
      "id": "example/1.png/1660563214863653",
      "selfLink": "https://www.googleapis.com/storage/v1/b/example/o/1.png",
      "mediaLink": "https://content-storage.googleapis.com/download/storage/v1/b/example/o/1.png?generation=1660563214863653&alt=media",
      "name": "1.png",
      "bucket": "example",
      "generation": "1660563214863653",
      "metageneration": "1",
      "contentType": "image/png",
      "storageClass": "STANDARD",
      "size": "56535",
      "md5Hash": "fHcEH1vPwA6eTPqxuasXcg==",
      "crc32c": "j/un9g==",
      "etag": "CKWasoTgyPkCEAE=",
      "timeCreated": "2022-08-15T11:33:34.866Z",
      "updated": "2022-08-15T11:33:34.866Z",
      "timeStorageClassUpdated": "2022-08-15T11:33:34.866Z"
    },
    {
      "kind": "storage#object",
      "id": "example/2.png/1660563214883337",
      "selfLink": "https://www.googleapis.com/storage/v1/b/example/o/2.png",
      "mediaLink": "https://content-storage.googleapis.com/download/storage/v1/b/example/o/2.png?generation=1660563214883337&alt=media",
      "name": "2.png",
      "bucket": "example",
      "generation": "1660563214883337",
      "metageneration": "1",
      "contentType": "image/png",
      "storageClass": "STANDARD",
      "size": "45506",
      "md5Hash": "e6LsGusU7pFJZk+114NV1g==",
      "crc32c": "L00QAg==",
      "etag": "CIm0s4TgyPkCEAE=",
      "timeCreated": "2022-08-15T11:33:34.886Z",
      "updated": "2022-08-15T11:33:34.886Z",
      "timeStorageClassUpdated": "2022-08-15T11:33:34.886Z"
    }
  ]
}
    "#;
}
