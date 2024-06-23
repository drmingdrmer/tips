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
- **pandoc**
  pandoc list format:
  `pandoc --list-output-formats`
-
- Markdown render:
  rust:
	- https://github.com/rust-lang/mdBook
	- https://github.com/wooorm/markdown-rs
-
- ### Markdown convert
	- Convert wikipedia to markdown:
	  click `[edit]` and copy the wiki source code in mediawiki
	  `cat wiki.txt | pandoc --from mediawiki  --no-highlight --to markdown > wiki.md`
-
-
- github-markdown.css
  https://github.com/sindresorhus/github-markdown-css
-
- ### Markdown TOC(table of content)
	- **doctoc**
	  https://github.com/thlorenz/doctoc
	  #install `npm install -g doctoc`
	  #usage `doctco .` to generate for all in dir;
	  javascript; support in-place build. Used in `consensus-essence`
	- **markdown-toc**
	  https://github.com/jonschlinkert/markdown-toc#cli
	  #install `brew install markdown-toc`
	  javascript; just output toc, no update inplace; good for rust doc;
	  Used in `openraft` to build faq TOC.
	- crate: **markdown-toc** `md-toc`
	  https://crates.io/crates/markdown-toc
	  #install `cargo install markdown-toc`
	  #usage `md-toc README.md`
	  rust; support `--min-depth 1` super good!
	- **github-markdown-toc**
	  https://github.com/ekalinin/github-markdown-toc
	  golang; looks very popular
- ### Markdown link check
	- **lychee** https://github.com/lycheeverse/lychee
	  Used by databend; support cache.
	  #install  `cargo install lychee`
	  Sample usage with cache and issue report:
	  https://github.com/datafuselabs/databend/blob/aa54ea223932087a7b1a06fb690ed780aad8ca5f/.github/workflows/links.yml
	  ```yaml
	        - name: Restore lychee cache
	          id: restore-cache
	          uses: actions/cache/restore@v3
	          with:
	            path: .lycheecache
	            key: cache-lychee-${{ github.sha }}
	            restore-keys: cache-lychee-
	  
	        - name: Link Checker
	          id: lychee
	          uses: lycheeverse/lychee-action@v1.8.0
	          with:
	            args: "--base . --cache --max-cache-age 1d . --exclude 'https?://twitter\\.com(?:/.*$)?$'"
	  
	        - name: Save lychee cache
	          uses: actions/cache/save@v3
	          if: always()
	          with:
	            path: .lycheecache
	            key: ${{ steps.restore-cache.outputs.cache-primary-key }}
	  
	        - name: Create Issue From File
	          if: env.lychee_exit_code != 0
	          uses: peter-evans/create-issue-from-file@v4
	          with:
	            title: Link Checker Report
	            content-filepath: ./lychee/out.md
	            labels: report, automated issue
	  ```
	- **markdown-link-check**
	  https://github.com/tcort/markdown-link-check
	  #install `npm install -g markdown-link-check`
	  #usage `markdown-link-check ./README.md`
	  #usage `find . -name \*.md -print0 | xargs -0 -n1 markdown-link-check` recursive check;
	  javascript; looks good; support: `base-url`, config file to specify pattern;
	- **github-action-markdown-link-check**
	  https://github.com/gaurav-nelson/github-action-markdown-link-check
	  Action wrapper of markdown-link-check
	- **mdbook-linkcheck**
	  https://github.com/Michael-F-Bryan/mdbook-linkcheck
	  rust;  mdbook plugin
-