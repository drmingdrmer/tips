tags:: tips, rust-programming, cargo

软件有升级时, 需要提供一个升级程序, 读旧数据, 转换, 写入新格式的数据.
所以要求它的代码中同时使用新旧两个版本的代码,
Cargo.toml 提供了方便的方法允许 main branch 中可以直接引用 `0.8.176` 版本中的数据定义:
其中, `git` 指定代码库的 url,
`tag` 指定引用哪个版本,
`package` 用于指定引用这个 repo 中的哪个 crate.

#### Branch `main`

```text
# cat Cargo.toml
[dependencies]
...
meta-app-v1 = {
    git = "https://github.com/datafuselabs/databend", # -.
    tag = "v0.8.176",                                 #  |
    package = "common-meta-app" # --------------.        |
}                                               |        |
                                                |        |
                                                |        |
# cat src/lib.rs                                |        |
...                                             |        |
use meta_app_v1::schema::TableMeta; // ----.    |        |
                                           |    |        |
```

#### Tag `v0.8.176`

```text
                                           |    |        |
./databend/ <------------------------------|----|--------'
...                                        |    |
▾ src/                                     |    |
  ▾ meta/                                  |    |
    ▾ app/                                 |    |
      ▾ src/                               |    |
        ▾ schema/                          |    |
             mod.rs                        |    |
             table.rs                      |    |
               # pub struct TableIdent { <-'    |
               #     pub table_id: u64,         |
               # }                              |
          lib.rs                                |
        Cargo.toml                              |
          # [package]                           |
          # name = "common-meta-app" <----------'
          # ...
```

