### Categories
	- [[rust-useful-crate]]
	- [[rust-programming]]
	- [[rust-search-engine]]
	- [[rust-serialization]]
	- [[mdbook]]
	- [[rust-learn]]
-
- **cargo config**
  https://doc.rust-lang.org/cargo/reference/config.html
- [[Rust 如何定义错误]]
- https://docs.rs/plotters/latest/plotters/
-
- serde dyn Trait trait object:
  https://github.com/dtolnay/typetag
-
- **hyperbridge** Fast multi-producer, multi-consumer unbounded channel with async support.
  https://github.com/singaraiona/hyperbridge
-
-
- Add katex math to docs.rs
  https://github.com/drmingdrmer/math-in-rust-doc
  https://docs.rs/katex-doc/latest/katex_doc/
  https://stackoverflow.com/questions/46495063/how-to-write-math-formulas-for-rust-documentation
-
- tokio doc
	- https://docs.rs/tokio/latest/tokio/#cpu-bound-tasks-and-blocking-code
	- https://tokio.rs/tokio/topics/bridging
-
- ### error-stack
	- https://github.com/datafuselabs/databend/issues/15741
	- https://zhuanlan.zhihu.com/p/696420829
	- https://crates.io/crates/error-stack
	- https://docs.rs/error-stack/0.4.1/error_stack/trait.ResultExt.html#tymethod.change_context
	- https://hash.dev/blog/announcing-error-stack
	- https://rust-lang.github.io/rfcs/3192-dyno.html?
-
- ### log crate
	- https://docs.rs/log/0.4.21/log/
	- https://docs.rs/log/0.4.21/log/kv/index.html
	-
-
- 把 Rust 当作（伪）脚本用。
	- ```
	  #!/bin/sh
	  #![allow(unused_attributes)] /*
	  OUT=/tmp/tmp && rustc "$0" -o ${OUT} && exec ${OUT} $@ || exit $? #*/
	  
	  use std::process::Command;
	  use std::io::Result;
	  use std::path::PathBuf;
	  use std::fs;
	  
	  fn mkdir(dir_name: &str) -> Result<()> {
	      fs::create_dir(dir_name)
	  }
	  
	  fn main() {
	      println!("hello");
	  }
	  ```
	- 重点是前三行。写的很巧妙，既是合法的 rust 语法，又是合法的 shell 语法。
	  第一行 shebang 指定了用 sh 执行该脚本，而 rustc 本身也会忽略 shebang 行，没啥好说的。
	  第二行声明了 rust 的一个内部属性，因为语法是 `#` 开头，所以刚好也是 shell 语法的注释行。结尾用了 rust 的多行注释，完全忽略第三行。
	  而 shell 免疫 rust 的多行注释，所以第三行正常执行。其实就是用 rustc 编译当前文件到二进制程序再执行并退出。
	  剩下的部分就是 rust 的逻辑了。
	- 作者：无色明天
	  链接：https://www.zhihu.com/question/282113351/answer/3622421263
	  来源：知乎
	  著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。