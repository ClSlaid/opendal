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

use std::{
    io::{Error, ErrorKind, Result},
    pin::Pin,
    task::{Context, Poll},
};

use futures::{ready, Stream};
use pin_project::pin_project;
use tikv_client::{Config, Key, KvPair, TransactionClient};

use crate::{
    adapters::kv::Adapter,
    error::{new_other_backend_error, new_other_object_error},
    ops::Operation,
    path::normalize_root,
};

const DEFAULT_TIKV_ENDPOINT: &str = "127.0.0.1:2379";
const DEFAULT_TIKV_PORT: u16 = 6379;

/// TiKV backend builder
#[derive(Clone, Default)]
pub struct Builder {
    /// network address of the TiKV service.
    ///
    /// default is "127.0.0.1:2379"
    endpoints: Option<Vec<String>>,
    /// whether using insecure connection to TiKV
    insecure: bool,
    /// certificate authority file path
    ca_path: Option<String>,
    /// cert path
    cert_path: Option<String>,
    /// key path
    key_path: Option<String>,

    /// the working directory of the TiKV service. Can be "path/to/dir"
    ///
    /// default is "/"
    root: Option<String>,
}

impl Builder {
    pub fn endpoints(&mut self, endpoints: impl Into<Vec<&str>>) -> &mut Self {
        let ep: Vec<String> = endpoints.into().into_iter().map(|s| s.to_owned()).collect();
        if !ep.is_empty() {
            self.endpoints = Some(ep)
        }
        self
    }

    pub fn insecure(&mut self) -> &mut Self {
        self.insecure = true;
        self
    }

    pub fn ca_path(&mut self, ca_path: &str) -> &mut Self {
        if !ca_path.is_empty() {
            self.ca_path = Some(ca_path.to_string())
        }
        self
    }

    pub fn cert_path(&mut self, cert_path: &str) -> &mut Self {
        if !cert_path.is_empty() {
            self.cert_path = Some(cert_path.to_string())
        }
        self
    }

    pub fn key_path(&mut self, key_path: &str) -> &mut Self {
        if !key_path.is_empty() {
            self.key_path = Some(key_path.to_string())
        }
        self
    }

    pub fn root(&mut self, root: &str) -> &mut Self {
        if !root.is_empty() {
            self.root = Some(root.to_string())
        }
        self
    }
}

impl Builder {
    pub async fn build(&mut self) -> Result<Backend> {
        let endpoints = self
            .endpoints
            .clone()
            .unwrap_or_else(|| vec![DEFAULT_TIKV_ENDPOINT.to_string()]);

        let r = self
            .root
            .clone()
            .unwrap_or_else(|| "/".to_string())
            .as_str();
        let root = normalize_root(r);

        let mut ctx = Hashmap::from([("endpoints".to_string(), format!("{:?}", endpoint.clone()))]);

        let client = if self.insecure {
            TransactionClient::new(endpoints).await.map_err(|err| {
                new_other_backend_error(ctx.clone(), anyhow::anyhow!("invalid configuration", err))
            })?
        } else if self.ca_path.is_some() && self.key_path.is_some() && self.cert_path.is_some() {
            let (ca_path, key_path, cert_path) = (
                self.ca_path.clone().unwrap(),
                self.key_path.clone().unwrap(),
                self.cert_path.clone().unwrap(),
            );
            ctx.extend([
                ("ca_path".to_string(), ca_path.clone()),
                ("key_path".to_string(), key_path.clone()),
                ("cert_path".to_string(), cert_path.clone()),
            ]);
            let config = Config::default().with_security(ca_path, cert_path, key_path);
            Transaction::new_with_config(endpoints, config)
                .await
                .map_err(|err| {
                    new_other_backend_error(
                        ctx.clone(),
                        anyhow::anyhow!("invalid configuration", err),
                    )
                })?
        } else {
            return Err(new_other_backend_error(
                ctx.clone(),
                anyhow::anyhow!("invalid configuration: no enough certifications"),
            ));
        };

        debug!("backend build finished: {:?}", &self);
        Ok(Backend::new(Adapter {
            client,
            next_id: Arc::new(AtomicU64::new(0)),
        }))
    }
}

/// Backend for TiKV service
pub type Backend = kv::Backend<Adapter>;

#[derive(Clone)]
pub struct Adapter {
    client: TransactionClient,
    next_id: Arc<AtomicU64>,
}

#[async_trait::async_trait]
impl kv::Adapter for Adapter {
    fn metadata(&self) -> kv::Metadata {
        kv::Metadata::new(
            Scheme::TiKV,
            "TiKV",
            AccessorCapability::Read | AccessorCapability::Write,
        )
    }

    async fn next_id(&self) -> Result<u64> {
        Ok(self.next_id.fetch_add(1, Ordering::Relaxed))
    }

    async fn set(&self, key: &[u8], value: &[u8]) -> Result<()> {
        let mut txn = self
            .client
            .begin_optimistic()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, anyhow!("tikv: {:?}", e)))?;

        txn.put(key, value)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, anyhow!("tikv: {:?}", e)))
    }

    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let mut txn = self
            .client
            .begin_optimistic()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, anyhow!("tikv: {:?}", e)))?;

        txn.get(key)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, anyhow!("tikv: {:?}", e)))
    }

    async fn delete(&self, key: &[u8]) -> Result<()> {
        let mut txn = self
            .client
            .begin_optimistic()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, anyhow!("tikv: {:?}", e)))?;

        txn.delete(key)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, anyhow!("tikv: {:?}", e)))
    }

    async fn scan(&self, prefix: &[u8]) -> Result<kv::KeyStreamer> {
        let mut txn = self
            .client
            .begin_optimistic()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, anyhow!("tikv: {:?}", e)))?;

        let it = txn
            .scan_keys(prefix)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, anyhow!("tikv: {:?}", e)))?;
        Ok(kv::KeyStreamer::new(it))
    }
}

#[pin_project]
struct KeyStream {
    iter: Box<impl Iterator<Item = Key>>,
}

impl KeyStream {
    fn new(it: impl Iterator<Item = Key>) -> Self {
        Self { iter: Box::new(it) }
    }
}

impl Stream for KeyStream {
    type Item = Result<Vec<u8>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.iter.next() {
            Some(k) => Poll::Ready(Some(Ok(k.into()))),
            None => Poll::Ready(None),
        }
    }
}
