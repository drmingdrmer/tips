- **match_template** This crate provides a macro that can be used to append a match expression with multiple arms
  ```
  match_template! {
    T = [Int, Real, Double],
    match Foo {
        EvalType::T => { panic!("{}", EvalType::T); },
        EvalType::Other => unreachable!(),
    }
  }
  ```
  generates
  ```
  match Foo {
    EvalType::Int => { panic!("{}", EvalType::Int); },
    EvalType::Real => { panic!("{}", EvalType::Real); },
    EvalType::Double => { panic!("{}", EvalType::Double); },
    EvalType::Other => unreachable!(),
  }
  ```
  https://tikv.github.io/doc/match_template/macro.match_template.html