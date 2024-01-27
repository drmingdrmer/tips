- **markdown-link-check**
  找出坏链接 https://github.com/tcort/markdown-link-check 
  #npm #markdown #link
	- #install `npm install -g markdown-link-check`
	- `find . -name \*.md -print0 | xargs -0 -n1 markdown-link-check -c config.json`
	- `<!-- markdown-link-check-disable -->` disables markdown link check
	  github-action: https://github.com/gaurav-nelson/github-action-markdown-link-check
- **dashmap**
  DashMap is an implementation of a concurrent associative array/hashmap in Rust.
  https://docs.rs/dashmap/latest/dashmap/
  #rust #crate #concurrent #hashmap #reading
- **marp**
  is the ecosystem to write your presentation with plain Markdown.
  https://github.com/marp-team/marp
  #npm #markdown #slides #ppt
	- #install `npm install -g @marp-team/marp-cli`
	- #install-secondary-mac `brew install marp-cli`
	- #example: https://yhatt.github.io/marp-cli-example/
	- https://github.com/marp-team/marp-cli#npx
	  ```
	  # Convert slide deck into HTML
	  npx @marp-team/marp-cli@latest slide-deck.md
	  npx @marp-team/marp-cli@latest slide-deck.md -o output.html
	  
	  # Convert slide deck into PDF
	  npx @marp-team/marp-cli@latest slide-deck.md --pdf
	  npx @marp-team/marp-cli@latest slide-deck.md -o output.pdf
	  
	  # Convert slide deck into PowerPoint document (PPTX)
	  npx @marp-team/marp-cli@latest slide-deck.md --pptx
	  npx @marp-team/marp-cli@latest slide-deck.md -o output.pptx
	  
	  # Watch mode
	  npx @marp-team/marp-cli@latest -w slide-deck.md
	  
	  # Server mode (Pass directory to serve)
	  npx @marp-team/marp-cli@latest -s ./slides
	  ```
	- #tutorial https://sspai.com/post/55718
	- #tutorial https://zhuanlan.zhihu.com/p/149521766
- **moka**:
  A high performance concurrent caching library for Rust
  https://github.com/moka-rs/moka
- **pyo3**:
  https://pyo3.rs/v0.18.0
- **zellij**:
  Pluggable terminal workspace, with terminal multiplexer as the base feature
  #install `brea install zellij`
	- c-s e: scroll back editing
	- `zrf ls`: run command in a pane
	- c-p e: embed floating pane
	- **doc**: https://zellij.dev/documentation/zellij-edit.html
	- Sample layout
	  ```
	  // Usage:
	  //   zellij action new-tab --layout 3m1q.kdl
	  //   zellij --layout 3m1q.kdl
	  layout {
	  
	      tab name="foo" {
	          // tab-bar
	          pane size=1 borderless=true {
	              plugin location="zellij:tab-bar"
	          }
	  
	          pane split_direction="vertical" {
	              pane split_direction="horizontal" {
	                  pane {
	                      command "bash"
	                      args "-c" "./target/debug/databend-query -c xp-ben/config/query-1.toml --internal-enable-sandbox-tenant"
	                      start_suspended true
	                  }
	                  // pane
	              }
	              pane split_direction="horizontal" {
	                  pane  {
	                      command "bash"
	                      args "-c" "./target/debug/databend-meta -c xp-ben/config/meta-1.toml"
	                  }
	                  pane {
	                      command "bash"
	                      args "-c" "./target/debug/databend-meta -c xp-ben/config/meta-2.toml"
	                  }
	              }
	          }
	      }
	  
	      // status-bar
	      pane size=2 borderless=true {
	          plugin location="zellij:status-bar"
	      }
	  }
	  
	  ```
-
- **mprocs**
  Run multiple commands in parallel
  https://github.com/pvolok/mprocs
  #install `cargo install mprocs`
	- simple to use
-
- **hyper**
  terminal
  #install `brew install hyper`
	- Cmd-2 does not work. cmdIsMeta/altIsMeta does not work.
- emoji for everyone: https://github.com/twitter/twemoji
-
- **lsd**
  replacement of `ls`
  #install `cargo install lsd`
  https://github.com/lsd-rs/lsd
-
- **naugat-ocr**
  PDF to markdown, by facebook Meta AI
  #install `pip install nougat-ocr`
  https://facebookresearch.github.io/nougat/
  https://github.com/facebookresearch/nougat
- **marker**
  Convert PDF to markdown
  manually install, not tested yet
  https://github.com/VikParuchuri/marker
- Access patterns. Web cache workloads typically follow Power-law (generalized Zipfian) distributions [20, 26, 27, 34, 49, 52, 55, 81, 82, 97], 
  
  In detail, the ith popular object has a relative frequency of 1/iα, where α is a parameter that decides the skewness of the workload. Previous works find different α values from 0.6 to 0.8 [26], 0.56 [49], 0.71– 0.76 [51], 0.55–0.9 [20], and 0.6–1.5 [97]. 
  
  For example, MemC3 [47] uses Cuckoo hashing and CLOCK
  eviction to improve Memcached’s throughput and scalabil- ity;
  -- sieve paper
-
- **john**: password cracker
  https://github.com/openwall/john
-
- **AudioEdit** app store
  trim audio
- **Vary** AI OCR, latex, markdown:
  https://github.com/Ucas-HaoranWei/Vary?tab=readme-ov-file#Demo
  Demo: http://region-31.seetacloud.com:22701/
- **XcodeBenchmark** measures the compilation time of a large codebase on iMac, MacBook, and Mac Pro
  https://github.com/devMEremenko/XcodeBenchmark
- **FireDBG** Rust debugger
  https://firedbg.sea-ql.org/blog/2023-12-12-introducing-firedbg/