- typora
	- ✅ auto save image locally
	- ✅ paste html and convert to markdown
- logseq
- obsidian
- joplin
- **mdnice**
  https://editor.mdnice.com/
  convert md to zhihu format or else
- **marktext** mac app
  https://github.com/marktext/marktext
  recommended by Lei Wenfang
-
- Markdown render:
  rust:
	- https://github.com/rust-lang/mdBook
	- https://github.com/wooorm/markdown-rs
-
-
-
-
- github-markdown.css
  https://github.com/sindresorhus/github-markdown-css
-
- Markdown TOC(table of content)
	- **doctoc**
	  https://github.com/thlorenz/doctoc
	  #install `npm install -g doctoc`
	  #usage `doctco .` to generate for all in dir;
	  javascript; support in-place build. Used in `consensus-essence`
	- **markdown-toc**
	  https://github.com/jonschlinkert/markdown-toc#cli
	  #install `brew install markdown-toc`
	  javascript; just output toc, no update inplace; good for rust doc;
	  Used in openraft to build faq TOC.
	- crate: **markdown-toc** `md-toc`
	  https://crates.io/crates/markdown-toc
	  #install `cargo install markdown-toc`
	  #usage `md-toc README.md`
	  rust; support `--min-depth 1` super good!
	- **github-markdown-toc**
	  https://github.com/ekalinin/github-markdown-toc
	  golang; looks very popular