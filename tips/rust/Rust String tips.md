tags:: tips, rust-programming, string

- un-indents multiline string at compile time with crate `indoc`: 
  https://docs.rs/indoc/latest/indoc/
  ```rust
  use indoc::indoc;
  
  fn main() {
    let testing = indoc! {"
        def hello():
            print('Hello, world!')
  
        hello()
    "};
    let expected = "def hello():\n    print('Hello, world!')\n\nhello()\n";
    assert_eq!(testing, expected);
  }
  ```