tags:: tips, rust-programming, cargo, dep

`Cargo.toml` dependency 按照类别来管理, 以下是我自己建立的一些常用类别:

```toml
# data flow
futures         = { workspace = true }
itertools       = { workspace = true }

# data structure
binary-heap-plus= { workspace = true }
lru-cache       = { workspace = true }
compare         = { workspace = true }

# error handling
anyhow          = { workspace = true }

# serialization
byteorder       = { workspace = true }
bytes           = { workspace = true }
crc32fast       = { workspace = true }
serde           = { workspace = true }
serde_json      = { workspace = true }

# testing
maplit          = { workspace = true }
tempfile        = { workspace = true }
```
