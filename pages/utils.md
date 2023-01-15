- `npm install -g markdown-link-check`
  找出坏链接 https://github.com/tcort/markdown-link-check 
  `find . -name \*.md -print0 | xargs -0 -n1 markdown-link-check -c config.json`
  `<!-- markdown-link-check-disable -->` disables markdown link check
   #npm #markdown #link