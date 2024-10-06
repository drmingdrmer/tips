- https://rust-lang.github.io/mdBook/
- #install `cargo install mdbook`
- #usage `mdbook build` `mdbook serve`
-
- Add tags: https://crates.io/crates/mdbook-tagger
- add giscus comment: https://sakaketsukihana.github.io/Notes/mdBook-and-Giscus.html
- convert to latex and pdf: https://github.com/lbeckman314/mdbook-latex
- mathjax support: https://rust-lang.github.io/mdBook/format/mathjax.html
	- > **Note:** The usual delimiters MathJax uses are not yet supported. You can't currently use `$$ ... $$` as delimiters and the `\[ ... \]` delimiters need an extra backslash to work. Hopefully this limitation will be lifted soon.
	- > **Note:** When you use double backslashes in MathJax blocks (for example in commands such as `\begin{cases} \frac 1 2 \\ \frac 3 4 \end{cases}`) you need to add *two extra* backslashes (e.g., `\begin{cases} \frac 1 2 \\\\ \frac 3 4 \end{cases}`).
- My mdbook building github-action
  https://github.com/drmingdrmer/mdbook-full/
- My svgbob fork:
  svgbob renders ascii graph into svg.
  mdbook-svgbob adopt svgbob in mdbook
  https://github.com/drmingdrmer/mdbook-svgbob/
-