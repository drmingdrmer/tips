- Add workflow: 
  ```
  name: Publish
  
  concurrency:
    group: ${{ github.workflow }}-${{ github.ref }}
    cancel-in-progress: true
  
  on:
    push:
      branches:
        - main
    workflow_dispatch:
  
  jobs:
    build:
      runs-on: ubuntu-latest
  
      steps:
        - uses: actions/checkout@v3
  
        - name: Logseq Publish
          # Using main branch to reproduce https://github.com/pengx17/logseq-publish/issues/8
          # Should switch back to stable release after this issue fixed.
          uses: pengx17/logseq-publish@main
          with:
            dest: www
  
        - name: add a nojekyll file
          # to make sure asset paths are correctly identified
          # -- ref: https://github.com/pengx17/logseq-publish#usage
          run: touch $GITHUB_WORKSPACE/www/.nojekyll
  
        # Ideas borrowed from https://logseq.abosen.top/#/page/%E9%85%8D%E7%BD%AE%20logseq%20%E8%87%AA%E5%8A%A8%E5%8F%91%E5%B8%83%E7%9B%B8%E5%85%B3%E6%B5%81%E7%A8%8B
        - name: Inject comments
          run: sed -i "s@</body>@$( cat scripts/comments.html | tr '\n' ' ' | sed 's@&@\\&@g' )</body>@"  www/index.html
  
        - name: Deploy 🚀
          uses: JamesIves/github-pages-deploy-action@v4
          with:
            branch: gh-pages
            folder: www
            clean: true
  
  ```
- Generate giscus script online: https://giscus.app; put the script to `scripts/comments.html`
- Enable **all pages public** in logseq setting.