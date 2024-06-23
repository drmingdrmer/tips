-
- Creates GitHub releases for pushed tags, with the commit log as release body https://github.com/fregante/release-with-changelog
-
- Useful badges:
  ```
  [![Discord Chat](https://img.shields.io/discord/1180545690976391251?logo=discord&style=flat-square)](https://discord.gg/fFPsTqYqUg)
  [![Crates.io](https://img.shields.io/crates/v/validit.svg)](https://crates.io/crates/validit)
  [![docs.rs](https://docs.rs/validit/badge.svg)](https://docs.rs/validit)
  ![Crates.io](https://img.shields.io/crates/d/validit.svg)
  ![Crates.io](https://img.shields.io/crates/dv/validit.svg)
  
  ```
  See: https://shields.io/badges/discord
-
- ### Github action
	- **cache** caching dependencies and build outputs to improve workflow execution time
	  https://github.com/actions/cache
	  Used by databend: https://github.com/datafuselabs/databend/blob/aa54ea223932087a7b1a06fb690ed780aad8ca5f/.github/workflows/links.yml
	  Support various languages: https://github.com/actions/cache/blob/main/examples.md#rust---cargo
	  Cache strategies: https://github.com/actions/cache/blob/main/caching-strategies.md
-