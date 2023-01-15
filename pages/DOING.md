- **Calatan number**
	- https://en.wikipedia.org/wiki/Catalan_number
		- https://zh.wikipedia.org/wiki/%E5%8D%A1%E5%A1%94%E5%85%B0%E6%95%B0
	- https://en.wikipedia.org/wiki/Binary_tree
	- 6.897: Advanced Data Structures (Spring 2003)
	  https://courses.csail.mit.edu/6.897/spring03/scribe_notes/
		- https://web.archive.org/web/20051124175104/http://theory.csail.mit.edu/classes/6.897/spring03/scribe_notes/L12/lecture12.pdf donwloaded
	- 「算法入门笔记」卡特兰数 
	  https://zhuanlan.zhihu.com/p/97619085
	- (Catalan 数列）
	  https://oi-wiki.org/math/combinatorics/catalan/
	- https://blog.csdn.net/u011080472/article/details/51162768
	- https://www.cnblogs.com/Morning-Glory/p/11747744.html
	- https://cloud.tencent.com/developer/article/1382271
	- https://cloud.tencent.com/developer/article/1679955
	- dyck words
	  https://en.wikipedia.org/wiki/Dyck_language
- Newton's_generalized_binomial_theorem
  https://en.wikipedia.org/wiki/Binomial_theorem#Newton's_generalized_binomial_theorem
-
- Databend 内幕大揭秘第一弹 - minibend 简介 https://mp.weixin.qq.com/s/8RGFYXu62V8QMoMiUxZJIg
-
-
- Read PR: feat: try to improve object storage io read https://github.com/datafuselabs/databend/pull/9335/files
-
- 纠删码? https://note.xuanwo.io/#/page/%E7%BA%A0%E5%88%A0%E7%A0%81
- Using a docker image to run a program? md2zhihu? ref: https://github.com/pengx17/logseq-publish
-
- 围观: A tutorial of building an LSM-Tree storage engine in a week! (WIP) : https://github.com/skyzh/mini-lsm #read
-
- **Openraft**
	- NOW nail `inflight`
	  :LOGBOOK:
	  CLOCK: [2023-01-10 Tue 09:15:13]
	  CLOCK: [2023-01-10 Tue 09:15:14]
	  :END:
	-
	- LATER raft read index?
	- LATER raft: build better mdbook
	- raft: 
	  ```
	  internal_server_state
	  ```
	  直接unwrap
	- raft: --feature bt is not necessary
	- raft: remove `RPCError::*NodeNotFound*
	- openraft: 需要serde 支持吗? 似乎不加serde也能运行memstore的测试?
	- openraft issue 617(Avoid double-wal) to doc
	- openraft: rocksdb example: flush is quick, unlike sled. why?
- rust:ssl: affected by env:  https://github.com/datafuselabs/openraft/issues/550
- ssl/tls: https://medium.com/double-pointer/ssl-vs-tls-vs-mtls-f5e836fe6b6d
	- https://docs.rs/rustls/0.20.7/rustls/ #read the doc
		- https://crates.io/crates/ring
		- mio
	- https://jbp.io/2019/07/01/rustls-vs-openssl-performance.html
	- https://docs.rs/rustwt/1.0.1/rustwt/index.html depends on openssl?
	- ed25519: https://www.cnblogs.com/librarookie/p/15389876.htm
	- es256:
- Faster string searching? https://docs.rs/memchr/latest/memchr/
- setup tls server:  fuse-query/src/common/http/src/http_shutdown_handlers.rs
- unify Paxos and 2pc: consensus-essence
- https://github.com/pingcap/awesome-database-learning
- ```
  upsert_table_copied_file_info
  ```
- **Write**
	- paxos-2pc: draft in: consensus-essence
	- write: https://databend.rs/doc/deploy/upgrade/compatibility
	  I've written docs: https://github.com/datafuselabs/fuse-query/blob/277b354239d244612689b155ba9e99d97336ecd9/src/meta/proto-conv/src/lib.rs#L39
	- databend version control
	- raft paper: 这一页讲了什么? 解释term+log的结构
	- 请问分布式事务一致性与raft或paxos协议解决的一致性问题是同一回事吗？
	  https://www.zhihu.com/question/275845393/answer/2845528855
- Windows Azure Storage: A Highly Available Cloud Storage Service With Strong Consistency
	- from: https://note.xuanwo.io/#/page/windows%20azure%20storage%3A%20a%20highly%20available%20cloud%20storage%20service%20with%20strong%20consistency
	- https://fuzhe1989.github.io/2021/05/02/windows-azure-storage-a-highly-available-cloud-storage-service-with-strong-consistency/
	- https://azure.microsoft.com/en-us/blog/sosp-paper-windows-azure-storage-a-highly-available-cloud-storage-service-with-strong-consistency/?cdn=disable
	- https://www.cs.purdue.edu/homes/csjgwang/CloudNativeDB/AzureStorageSOSP11.pdf
- async-trait-fn
	- https://github.com/wvwwvwwv/async-trait-fn/pull/9/files
-
-
- 失眠使用褪黑素?
- A categorized list of C++ resources. https://github.com/MattPD/cpplinks
- needs_drop(MaybeUninit): https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6f6dbcf23431645d8fed874b1c5c1366
- 常见的Rust lifetime误解 https://zhuanlan.zhihu.com/p/165976086
- apache iceberg
- decoupled metastore https://cloud.google.com/bigquery/docs/iceberg-tables
- learn: lotusdb, rosedb https://github.com/flower-corp/lotusdb https://github.com/flower-corp/rosedb https://github.com/flower-corp/minidb
- Learn a real kv store: Fast key-value DB in Go. https://github.com/dgraph-io/badger
	- ![image.png](../assets/image_1672801264721_0.png)
- rust: lru: https://github.com/jeromefroe/lru-rs
- https://docs.rs/opentelemetry-prometheus/0.11.0/opentelemetry_prometheus/
- Clearing/Expiration of old values in histogram/limit to histogram https://github.com/metrics-rs/metrics/issues/245#issuecomment-974155299
	- https://github.com/metrics-rs/metrics/blob/main/metrics-exporter-prometheus/src/registry.rs
	- https://github.com/metrics-rs/metrics/blob/main/metrics-util/src/histogram.rs
- 可以同时存在，但权益证明机制下运行的以太坊采用了经过更新的分叉选择算法，衡量链的“权重”。 权重是验证者投票的累积总数，并以验证者质押的以太币余额加权。
  因此共识是只有权重最大的链才是正确的链 -- 以太坊: https://twitter.com/brucknerwang/status/1610627658928521217
- https://editorconfig.org/
- tokio: https://github.com/tremor-rs/tremor-runtime/pull/2149/files
- Mac OSX终端走shadowsocks代理 https://github.com/mrdulin/blog/issues/18
- codegen::writes::codegen_arithmetic_type_v2();
- feat(query): efficiently memory two level group by in standalone mode https://github.com/datafuselabs/databend/pull/9504/files
-
- Databend:
	- LATER Tracking issue for new expression framework https://github.com/datafuselabs/databend/issues/6547
	- **QueryTeam  in 2023 feishu**
	  https://datafuselabs.feishu.cn/wiki/wikcnD0ymLV4t9j9T4av9ik3y8c
	- **2023 Roadmap**
	  https://github.com/datafuselabs/databend/issues/9448
	- DOING Tracking: issues about new expression https://github.com/datafuselabs/databend/issues/9480
	  :LOGBOOK:
	  CLOCK: [2023-01-07 Sat 08:51:31]
	  :END:
		- **Save the new schema format TableSchema directly to meta-service, without converting back to obsolete DataSchema.** https://github.com/datafuselabs/databend/issues/9498
			- PR: https://github.com/datafuselabs/databend/pull/9506
				- remove Interval type from test
		- **feat(meta-rewrite): meta data upgrade program: metarewrite** https://github.com/datafuselabs/databend/pull/9489
		-
- Add opendal streaming read to databend:
	- databend bump opendal: https://github.com/datafuselabs/databend/pull/9503/files
	- diff: https://github.com/datafuselabs/opendal/compare/v0.23.0...v0.24.0
-
- 3561d1be2d
  3561d1be2d644f98163b94e551db5004e7ba9dc8
- Rust implementation of the H3 geospatial indexing system. https://github.com/HydroniumLabs/h3o
- Memo: 并行测试hang问题分析 https://datafuselabs.feishu.cn/wiki/wikcnb1xe8HluJNi73WEcKDucj5
- An interpreter for Rust's mid-level intermediate representation
  https://github.com/rust-lang/miri
-
- mp TODO
	- remove title
	- remove tag
	- add id card
	- add cover
	- add source link
	- add footer qrcode
-
- Binomial Coefficients
  http://webcache.googleusercontent.com/search?q=cache:-R5Bj6CoHzEJ:poj.org/problem%3Fid%3D3219+&cd=2&hl=en&ct=clnk&gl=hk
- Distributed system
	- 分布式系统一致性的发展历史 (一)
	  https://danielw.cn/history-of-distributed-systems-1
	- https://danielw.cn/
	-