- Automatically commit changes made in your workflow run directly to your repo: https://github.com/EndBug/add-and-commit
- ```
        - uses: baptiste0928/cargo-install@v1
          with:
            crate: typos-cli
            args: --locked
            cache-key: typos-check
  ```