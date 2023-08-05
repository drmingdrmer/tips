tags:: tips, rust-programming, serde, msgpack, bincode


[bincode][] serialize 时不带meta信息, 如果数据格式变化, 无法提供向后兼容.
msgpack([rmp-serde][]) 可选的加入meta信息, 可以提供向后兼容.
兼容性测试代码: [serde-msgpack-bincode.rs](../rust-playground/src/bin/serde-msgpack-bincode.rs)


### 常用的序列化库的 benchmark


| Crate                            | Serialize ↑ | Deserialize | Size    | Zlib   | Zstd   |
|---                               |--:          |--:          |--:      |--:     |--:     |
| [bincode 1.3.3][bincode]         | 697.27 µs   | 4.2059 ms   | 1045784 | 373127 | 311761 |
| [prost 0.11.9][prost]            | 673.57 µs   | 5.0876 ms   | 764951  | 268137 | 227947 |
| [msgpacker 0.4.3][msgpacker]     | 1.5924 ms   | 4.5680 ms   | 764996  | 315291 | 264898 |
| [rmp-serde 1.1.2][rmp-serde]     | 1.6095 ms   | 5.8322 ms   | 784997  | 325384 | 278219 |
| [serde_cbor 0.11.2][serde_cbor]  | 2.4884 ms   | 8.2101 ms   | 1407835 | 403440 | 324081 |
| [bson 2.6.0][bson]               | 3.4703 ms   | 12.831 ms   | 1924682 | 532821 | 376270 |
| [serde_json 1.0.103][serde_json] | 5.9847 ms   | 11.408 ms   | 1827461 | 470560 | 361090 |

[完整的benchmark](https://github.com/djkoloski/rust_serialization_benchmark)

### bincode 对 serde 的支持不完整

参考 bincode 的 rc 版本的说明:
https://docs.rs/bincode/2.0.0-rc.3/bincode/serde/index.html#known-issues

> Because bincode is a format without meta data, there are several known issues
> with serde’s attributes. Please do not use any of the following attributes if
> you plan on using bincode, or use bincode’s own `derive` macros.
> 
> - `#[serde(flatten)]`
> - `#[serde(skip)]`
> - `#[serde(skip_deserializing)]`
> - `#[serde(skip_serializing)]`
> - `#[serde(skip_serializing_if = "path")]`
> - `#[serde(tag = "...")]`
> - `#[serde(untagged)]`
> 
> **Using any of the above attributes can and will cause issues with bincode and
> will result in lost data**. Consider using bincode’s own derive macro instead.

Databend 存储层的 metadata 因为兼容性问题已经从 bincode 切换到 msgpack:
https://github.com/datafuselabs/databend/pull/11592

### 其他序列化库的 benchmark

这些也包含在了: [完整的benchmark](https://github.com/djkoloski/rust_serialization_benchmark)

| Crate                                          | Serialize | Deserialize                                                                                                    | Size    | Zlib   | Zstd   |
|---                                             |--:        |--:                                                                                                             |--:      |--:     |--:     |
| [speedy 0.8.6][speedy]                         | 287.97 µs | 3.4328 ms                                                                                                      | 885780  | 362204 | 286514 |
| [abomonation 0.7.3][abomonation]               | 296.78 µs | <span title="unvalidated">*2.9852 ms\**</span>                                                                 | 1705800 | 530419 | 403304 |
| [alkahest 0.1.5][alkahest]                     | 300.49 µs | †                                                                                                              | 1045784 | 454157 | 389424 |
| [savefile 0.16.0][savefile]                    | 313.54 µs | 3.9789 ms                                                                                                      | 1045800 | 373139 | 311755 |
| [rkyv 0.7.42][rkyv]                            | 390.74 µs | <span title="unvalidated">*3.2543 ms\**</span> <span title="validated upfront with error">*4.2306 ms\**</span> | 1011488 | 383862 | 333545 |
| [nanoserde 0.1.33][nanoserde]                  | 409.54 µs | 4.0285 ms                                                                                                      | 1045784 | 373127 | 311761 |
| [postcard 1.0.6][postcard]                     | 517.78 µs | 4.2953 ms                                                                                                      | 724953  | 302399 | 253747 |
| [borsh 0.10.3][borsh]                          | 560.76 µs | 4.1540 ms                                                                                                      | 885780  | 362204 | 286514 |
| [bitcode 0.4.0][bitcode]                       | 610.09 µs | 4.1721 ms                                                                                                      | 703664  | 317711 | 273622 |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 720.58 µs | 4.5589 ms                                                                                                      | 765778  | 311743 | 264518 |
| [capnp 0.16.1][capnp]                          | 904.26 µs | †                                                                                                              | 1443216 | 513986 | 428649 |
| [serde_bare 0.5.0][serde_bare]                 | 969.75 µs | 4.0761 ms                                                                                                      | 765778  | 311715 | 264630 |
| [ciborium 0.2.1][ciborium]                     | 4.3867 ms | 14.290 ms                                                                                                      | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn]                             | 1.0118 ms | 4.7915 ms                                                                                                      | 724953  | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers]             | 2.7901 ms | †                                                                                                              | 1276368 | 468539 | 388832 |
| [nachricht-serde 0.4.0][nachricht-serde]       | 9.0815 ms | 7.0720 ms                                                                                                      | 818669  | 332556 | 285514 |
| [simd-json 0.9.2][simd-json]                   | 2.8277 ms | 7.2679 ms                                                                                                      | 1827461 | 470560 | 361090 |
| [ron 0.8.0][ron]                               | 25.623 ms | 24.925 ms                                                                                                      | 1607459 | 449158 | 349713 |




[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.4.0
[borsh]: https://crates.io/crates/borsh/0.10.3
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.16.1
[ciborium]: https://crates.io/crates/ciborium/0.2.1
[dlhn]: https://crates.io/crates/dlhn/0.1.6
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.33
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.4
[postcard]: https://crates.io/crates/postcard/1.0.6
[prost]: https://crates.io/crates/prost/0.11.9
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.0
[savefile]: https://crates.io/crates/savefile/0.16.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.103
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6
