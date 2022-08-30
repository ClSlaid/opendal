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

use std::fmt::Debug;
use std::io::IoSliceMut;
use std::io::Result;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

use async_trait::async_trait;
use futures::AsyncRead;
use tracing::Span;

use crate::multipart::ObjectPart;
use crate::ops::OpAbortMultipart;
use crate::ops::OpCompleteMultipart;
use crate::ops::OpCreate;
use crate::ops::OpCreateMultipart;
use crate::ops::OpDelete;
use crate::ops::OpList;
use crate::ops::OpPresign;
use crate::ops::OpRead;
use crate::ops::OpStat;
use crate::ops::OpWrite;
use crate::ops::OpWriteMultipart;
use crate::ops::PresignedRequest;
use crate::Accessor;
use crate::AccessorMetadata;
use crate::BytesReader;
use crate::DirEntry;
use crate::DirStreamer;
use crate::Layer;
use crate::ObjectMetadata;

/// TracingLayer will add tracing for OpenDAL.
///
/// # Examples
///
/// ```
/// use anyhow::Result;
/// use opendal::layers::TracingLayer;
/// use opendal::Operator;
/// use opendal::Scheme;
///
/// let _ = Operator::from_env(Scheme::Fs)
///     .expect("must init")
///     .layer(TracingLayer);
/// ```
pub struct TracingLayer;

impl Layer for TracingLayer {
    fn layer(&self, inner: Arc<dyn Accessor>) -> Arc<dyn Accessor> {
        Arc::new(TracingAccessor { inner })
    }
}

pub struct TracingReader {
    span: Span,
    inner: BytesReader,
}

impl TracingReader {
    pub(crate) fn make_reader(span: Span, reader: BytesReader) -> BytesReader {
        let r = Self {
            span,
            inner: reader,
        };
        Box::new(r)
    }
}

impl AsyncRead for TracingReader {
    #[tracing::instrument(parent=&self.span, skip(self))]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut (*self.inner)).poll_read(cx, buf)
    }

    #[tracing::instrument(parent=&self.span, skip(self))]
    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut (*self.inner)).poll_read_vectored(cx, bufs)
    }
}

pub struct TracingStreamer {
    span: Span,
    inner: DirStreamer,
}

impl TracingStreamer {
    pub(crate) fn make_streamer(span: Span, streamer: DirStreamer) -> DirStreamer {
        let s = Self {
            span,
            inner: streamer,
        };
        Box::new(s)
    }
}

impl futures::Stream for TracingStreamer {
    type Item = Result<DirEntry>;

    #[tracing::instrument(parent=&self.span, skip(self))]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut (*self.inner)).poll_next(cx)
    }
}

#[derive(Debug)]
struct TracingAccessor {
    inner: Arc<dyn Accessor>,
}

#[async_trait]
impl Accessor for TracingAccessor {
    #[tracing::instrument]
    fn metadata(&self) -> AccessorMetadata {
        self.inner.metadata()
    }

    #[tracing::instrument]
    async fn create(&self, args: &OpCreate) -> Result<()> {
        self.inner.create(args).await
    }

    #[tracing::instrument]
    async fn read(&self, args: &OpRead) -> Result<BytesReader> {
        self.inner
            .read(args)
            .await
            .map(|r| TracingReader::make_reader(Span::current(), r))
    }

    #[tracing::instrument(skip(r))]
    async fn write(&self, args: &OpWrite, r: BytesReader) -> Result<u64> {
        let r = TracingReader::make_reader(Span::current(), r);
        self.inner.write(args, r).await
    }

    #[tracing::instrument]
    async fn stat(&self, args: &OpStat) -> Result<ObjectMetadata> {
        self.inner.stat(args).await
    }

    #[tracing::instrument]
    async fn delete(&self, args: &OpDelete) -> Result<()> {
        self.inner.delete(args).await
    }

    #[tracing::instrument]
    async fn list(&self, args: &OpList) -> Result<DirStreamer> {
        self.inner
            .list(args)
            .await
            .map(|s| TracingStreamer::make_streamer(Span::current(), s))
    }

    #[tracing::instrument]
    fn presign(&self, args: &OpPresign) -> Result<PresignedRequest> {
        self.inner.presign(args)
    }

    #[tracing::instrument]
    async fn create_multipart(&self, args: &OpCreateMultipart) -> Result<String> {
        self.inner.create_multipart(args).await
    }

    #[tracing::instrument(skip(r))]
    async fn write_multipart(&self, args: &OpWriteMultipart, r: BytesReader) -> Result<ObjectPart> {
        let r = TracingReader::make_reader(Span::current(), r);
        self.inner.write_multipart(args, r).await
    }

    #[tracing::instrument]
    async fn complete_multipart(&self, args: &OpCompleteMultipart) -> Result<()> {
        self.inner.complete_multipart(args).await
    }

    #[tracing::instrument]
    async fn abort_multipart(&self, args: &OpAbortMultipart) -> Result<()> {
        self.inner.abort_multipart(args).await
    }
}
