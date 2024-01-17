var srcIndex = new Map(JSON.parse('[\
["opendal",["",[["docs",[["comparisons",[],["mod.rs"]],["internals",[],["accessor.rs","layer.rs","mod.rs"]],["rfcs",[],["mod.rs"]]],["concepts.rs","mod.rs"]],["layers",[],["async_backtrace.rs","await_tree.rs","blocking.rs","chaos.rs","complete.rs","concurrent_limit.rs","error_context.rs","immutable_index.rs","logging.rs","madsim.rs","metrics.rs","minitrace.rs","mod.rs","oteltrace.rs","prometheus.rs","prometheus_client.rs","retry.rs","throttle.rs","timeout.rs","tracing.rs","type_eraser.rs"]],["raw",[["adapters",[["kv",[],["api.rs","backend.rs","mod.rs"]],["typed_kv",[],["api.rs","backend.rs","mod.rs"]]],["mod.rs"]],["http_util",[],["body.rs","bytes_content_range.rs","bytes_range.rs","client.rs","error.rs","header.rs","mod.rs","multipart.rs","uri.rs"]],["oio",[["buf",[],["adaptive.rs","chunked_bytes.rs","mod.rs","write_buf.rs"]],["list",[],["api.rs","flat_list.rs","hierarchy_list.rs","mod.rs","page_list.rs","prefix_list.rs"]],["read",[],["api.rs","buffer_reader.rs","file_read.rs","futures_read.rs","into_read_from_stream.rs","into_streamable_read.rs","lazy_read.rs","mod.rs","range_read.rs","std_read.rs","tokio_read.rs"]],["stream",[],["api.rs","into_stream.rs","into_stream_from_reader.rs","mod.rs"]],["write",[],["api.rs","append_write.rs","block_write.rs","exact_buf_write.rs","mod.rs","multipart_write.rs","one_shot_write.rs","range_write.rs"]]],["cursor.rs","entry.rs","mod.rs"]],["tests",[],["mod.rs","read.rs","utils.rs","write.rs"]]],["accessor.rs","chrono_util.rs","enum_utils.rs","futures_util.rs","layer.rs","mod.rs","operation.rs","ops.rs","path.rs","path_cache.rs","rps.rs","serde_util.rs","std_io_util.rs","tokio_util.rs","version.rs"]],["services",[["alluxio",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["atomicserver",[],["backend.rs","mod.rs"]],["azblob",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["azdls",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["azfile",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["b2",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["cacache",[],["backend.rs","mod.rs"]],["chainsafe",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["cloudflare_kv",[],["backend.rs","error.rs","mod.rs"]],["cos",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["d1",[],["backend.rs","error.rs","mod.rs","model.rs"]],["dashmap",[],["backend.rs","mod.rs"]],["dbfs",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","reader.rs","writer.rs"]],["dropbox",[],["backend.rs","builder.rs","core.rs","error.rs","mod.rs","writer.rs"]],["etcd",[],["backend.rs","mod.rs"]],["foundationdb",[],["backend.rs","mod.rs"]],["fs",[],["backend.rs","lister.rs","mod.rs","writer.rs"]],["ftp",[],["backend.rs","err.rs","lister.rs","mod.rs","util.rs","writer.rs"]],["gcs",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","uri.rs","writer.rs"]],["gdrive",[],["backend.rs","builder.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["ghac",[],["backend.rs","error.rs","mod.rs","writer.rs"]],["gridfs",[],["backend.rs","mod.rs"]],["hdfs",[],["backend.rs","lister.rs","mod.rs","writer.rs"]],["hdfs_native",[],["backend.rs","error.rs","lister.rs","mod.rs","reader.rs","writer.rs"]],["http",[],["backend.rs","error.rs","mod.rs"]],["huggingface",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs"]],["ipfs",[],["backend.rs","error.rs","ipld.rs","mod.rs"]],["ipmfs",[],["backend.rs","builder.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["libsql",[],["backend.rs","error.rs","mod.rs"]],["memcached",[],["ascii.rs","backend.rs","mod.rs"]],["memory",[],["backend.rs","mod.rs"]],["mini_moka",[],["backend.rs","mod.rs"]],["moka",[],["backend.rs","mod.rs"]],["mongodb",[],["backend.rs","mod.rs"]],["mysql",[],["backend.rs","mod.rs"]],["obs",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["onedrive",[],["backend.rs","builder.rs","error.rs","graph_model.rs","lister.rs","mod.rs","writer.rs"]],["oss",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["pcloud",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["persy",[],["backend.rs","mod.rs"]],["postgresql",[],["backend.rs","mod.rs"]],["redb",[],["backend.rs","mod.rs"]],["redis",[],["backend.rs","mod.rs"]],["rocksdb",[],["backend.rs","mod.rs"]],["s3",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["seafile",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["sftp",[],["backend.rs","error.rs","lister.rs","mod.rs","utils.rs","writer.rs"]],["sled",[],["backend.rs","mod.rs"]],["sqlite",[],["backend.rs","mod.rs"]],["supabase",[],["backend.rs","core.rs","error.rs","mod.rs","writer.rs"]],["swift",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["tikv",[],["backend.rs","mod.rs"]],["upyun",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["vercel_artifacts",[],["backend.rs","builder.rs","error.rs","mod.rs","writer.rs"]],["webdav",[],["backend.rs","error.rs","lister.rs","mod.rs","writer.rs"]],["webhdfs",[],["backend.rs","error.rs","lister.rs","message.rs","mod.rs","writer.rs"]],["yandex_disk",[],["backend.rs","core.rs","error.rs","lister.rs","mod.rs","writer.rs"]]],["mod.rs"]],["types",[["operator",[],["blocking_operator.rs","builder.rs","metadata.rs","mod.rs","operator.rs","operator_functions.rs","operator_futures.rs"]]],["builder.rs","capability.rs","entry.rs","error.rs","list.rs","metadata.rs","mod.rs","mode.rs","reader.rs","scheme.rs","writer.rs"]]],["lib.rs"]]]\
]'));
createSrcSidebar();
