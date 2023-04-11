tags:: tips, rust-programming, trait, seal



提供一个pub的trait, 允许当做泛型参数使用但是不允许将它实现给任何struct.

在定义这个pub trait的crate里, 给它添加一个private的super trait:

```rust
mod private {
    pub trait Sealed {}
}

pub trait SealedTrait : private::Sealed {
    fn method(&self);
}
```

这样在其他crate里, 因为无法访问private的`private::Sealed`, 所以除了定义它的crate之外都不能实现trait `SealedTrait`:

```rust
struct DownstreamType {}

// ERROR: module `private` is private
impl upstream::private::Sealed for DownstreamType {}

// ERROR: the trait bound `DownstreamType: upstream::private::Sealed` is not satisfied
impl upstream::SealedTrait for DownstreamType {
    fn method(&self) {}
}
```



Original:

---

# A definitive guide to sealed traits in Rust

https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/



For the longest time, I thought that "sealed trait" in Rust was a singular concept implementable in one specific way. To prevent downstream crates from implementing your traits, you make the traits sealed — done, end of story. **I was wrong!** [It turns out there are multiple ways to seal traits](https://hachyderm.io/@epage/109820270237801122), forming a pleasant spectrum of options:



|                                   | downstream code can use it as a bound | downstream code can call its methods | downstream types can impl it |
| --------------------------------- | ------------------------------------- | ------------------------------------ | ---------------------------- |
| **pub trait**                     | ✅                                     | ✅                                    | ✅                            |
| **supertrait sealed trait**       | ✅                                     | ✅                                    | ❌                            |
| **method signature sealed trait** | ✅                                     | ❌                                    | ❌                            |
| **private trait**                 | ❌                                     | ❌                                    | ❌                            |

In fact, we'll see in a minute that even this isn't the full picture! Our choices can be even more fine-grained than what we see above:

- It's possible to prevent calling only a *subset* of the trait's methods.
- It's possible to allow downstream code to implement a trait, while preventing those implementations from overriding some of the trait's methods.

But first, what is a sealed trait, and why is sealing useful? Feel free to skip a section or two ahead if you're already familiar with this:

- [What are sealed traits?](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/#what-are-sealed-traits)
- [The trick for sealing traits](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/#the-trick-for-sealing-traits)
- [Sealing traits with a supertrait](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/#sealing-traits-with-a-supertrait)
- [Sealing traits via method signatures](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/#sealing-traits-via-method-signatures)
- [Allowing only *some* methods to be called](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/#allowing-only-some-methods-to-be-called)
- [Partially-sealed traits](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/#partially-sealed-traits)
- [The full matrix of possibilities](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/#the-full-matrix-of-possibilities)

## What are sealed traits?

A trait is *sealed* if it cannot be implemented outside of its own crate.

Why would we care about this?

Adding new methods to a trait is *usually* a major breaking change: all types that implement the trait must also implement the new methods. But what if we want to avoid needing a new major version?

There are two ways around it.

The first option is to add a default implementation for all the new methods in the trait. Then, any types that implement the trait will get that implementation "for free," and are also able to replace it with their own. This is sometimes reasonable, but not always. For example, there may not be a sensible default implementation to be added, and using `todo!()` as a default implementation would definitely raise some eyebrows.

The second option requires some forethought: when originally adding the trait, we must have made it *sealed*. Sealed traits can be implemented only by types within the current crate. Since no other crates can suffer breaking changes, this change isn't semver-major.

While there are multiple ways to seal a trait, they all rely on the same trick.

## The trick for sealing traits

At a high level, the trick for sealing traits is straightforward enough: make the trait implementation require a type that is only accessible within the current crate. Downstream crates won't be able to use that type, so they won't be able to implement the trait. Done!

In practice, a bit of nuance is required. Rust doesn't allow leaking private types in a crate's public API:

```rust
trait PrivateTrait {}

pub trait PublicTrait : PrivateTrait {}
```

produces ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c8a04e39ab23f400bbcb5d376372a6a2))

```
error[E0445]: private trait `PrivateTrait` in public interface
 --> src/lib.rs:3:1
  |
1 | trait PrivateTrait {}
  | ------------------ `PrivateTrait` declared as private
2 |
3 | pub trait PublicTrait : PrivateTrait {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private trait
```

To avoid errors like this, we'll have to make sure all the types in our trait's API are public.

But a Rust *type* can be public without its *name* being public. This distinction makes sealed traits possible.

## Sealing traits with a supertrait

Instead of using a private supertrait, let's use a *public* supertrait whose name is *not publicly exported*:Additional information, like best practices for using sealed traits, is available in [the Rust API guidelines](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed).

```rust
mod private {
    pub trait Sealed {}
}

pub trait SealedTrait : private::Sealed {
    fn method(&self);
}
```

Implementing `SealedTrait` for a type in the same crate is easy. First, implement `private::Sealed` for the type, then implement `SealedTrait` normally: ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=926d32c199d4fcdbf1cf86f3d9f9a1ab))

```rust
pub struct TypeThatImplsSealed;

impl private::Sealed for TypeThatImplsSealed {}

impl SealedTrait for TypeThatImplsSealed {
    fn method(&self) {}
}
```

Downstream crates aren't able to do this! While `Sealed` itself is public, it's defined in a *private* module and never re-exported. This means the type is public but its *name* is private.

Referring to `private::Sealed` from a downstream crate produces errors:

```rust
struct DownstreamType {}

// ERROR: module `private` is private
impl upstream::private::Sealed for DownstreamType {}
```

Attempting to implement `SealedTrait` directly in a downstream crate also fails:

```rust
struct DownstreamType {}

// ERROR: the trait bound `DownstreamType: upstream::private::Sealed` is not satisfied
impl upstream::SealedTrait for DownstreamType {
    fn method(&self) {}
}
```

But sealing the trait in this way doesn't prevent downstream code from calling its methods. The following code in a downstream crate works just fine:

```rust
fn use_sealed(value: impl upstream::SealedTrait) {
    value.method()
}
```

This is where the other way to seal traits comes in.

## Sealing traits via method signatures

Sometimes a trait has to be public, but we want to prevent downstream crates from calling its methods.[Here](https://github.com/rust-lang/rust/blob/044a28a4091f2e1a5883f7fa990223f8b200a2cd/library/core/src/error.rs#L89-L100) is a use case in Rust's built-in `Error` trait. We'll see more of this trait later in the blog post.

We'll use the same "unnamable types" Unnamable, meaning "not able to be named." I double-checked the spelling, and it's is indeed "unnamable" and not "unnameable."idea, but this time applied to method arguments instead of a supertrait:

```rust
mod private {
    pub struct Token;
}

pub trait SealedTrait {
    fn method(&self, _: private::Token);
}
```

`private::Token` is a unit struct, and as a zero-sized type (ZST) it won't incur any performance overhead. This is one of [many cool tricks](https://www.hardmo.de/article/2021-03-14-zst-proof-types.md) one can do with ZSTs to enforce a property at compile time with zero runtime cost.Being able to name a unit struct is sufficient to create its value, so code able to name `private::Token` can call the trait's method like this: ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=34eb425ec87dddc1ea34dbc4a6984597))

```rust
pub struct TypeThatImplsSealed;

impl SealedTrait for TypeThatImplsSealed {
    fn method(&self, _: private::Token) {
        // impl here
    }
}
```

Meanwhile, downstream code can both see and name the trait and its method, but cannot implement the trait nor call the method: This is the inverse of the earlier situation. Earlier we had public types that we couldn't name, and now we have a method we can name but whose name is effectively unusable, almost as if it were private.Interestingly, Rust does not prevent using values whose types cannot be named. It's perfectly okay for our example's `upstream` crate to publicly expose a function that returns a value of type `upstream::private::Token`. Downstream crates would then be able to use this returned value as an argument when calling `upstream::SealedTrait` methods, thereby unsealing the trait. This is [yet another example](https://predr.ag/tags/semver/) of why semver in Rust is tricky.

```rust
struct DownstreamType {}

impl upstream::SealedTrait for DownstreamType {
    // ERROR: module `private` is private
    fn method(&self, token: upstream::private::Token) {}
}


fn call_method(value: impl upstream::SealedTrait) {
    // ERROR: module `private` is private
    let token = upstream::private::Token;
    value.method(token);
}
```

As long as *at least one* *required method* on the trait takes an argument with an unnamable type, the trait is sealed and cannot be implemented by downstream crates.

That last sentence has two sneaky load-bearing phrases: "at least one" and "required method." Let's examine each of them in turn.

## Allowing only *some* methods to be called

The "priv-token" trick for sealing crates requires that *at least one* method take an argument with an unnamable type. What happens if the trait has other methods that don't take an unnamable type?

```rust
mod private {
    pub struct Token;
}

pub trait SealedTrait {
    fn callable_method(&self);

    fn non_callable_method(&self, _: private::Token);
}
```

There's nothing stopping those methods from being called from downstream crates. The following code works normally:

```rust
fn call_method(value: impl upstream::SealedTrait) {
    value.callable_method();
}
```

This allows us to explicitly decide *which methods* should be uncallable from downstream crates. This is a pattern used by Rust's standard library [in the definition of the `Error` trait](https://github.com/rust-lang/rust/blob/044a28a4091f2e1a5883f7fa990223f8b200a2cd/library/core/src/error.rs#L83-L100): the `source()` method can be called normally from anywhere, but the `type_id()` method cannot be called or overridden from downstream code since it asks for an argument of type `private::Internal`.

Now, you might be thinking: "I've definitely implemented the `Error` trait in my code ... It can't possibly be sealed?!" 🤔

And you're right! ✨

## Partially-sealed traits

The built-in `Error` trait is partially-sealed: downstream implementors can override *some but not all* of its methods. The trait provides a [default implementation](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations) for the methods that cannot be overridden, and downstream implementors *must* use that default implementation.

For example, the following code is able to override `Error::source()` just fine: ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f451b5517933700b49aef3bf25a9e4b3))

```rust
use std::fmt::{self, Display, Formatter};
use std::error::Error;

#[derive(Debug)]
struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "MyError")
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

But let's try to override `Error::type_id()`

```rust
impl Error for MyError {
    fn type_id(&self, _: core::error::private::Internal) -> std::any::TypeId {
        todo!()
    }
}
```

and we get: We also get some "unstable library feature" errors since the `type_id()` method is not stable. I have omitted them since they are not relevant to our discussion of partially-sealed traits.([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7574afca169c256cd9482ba4e0fe6c43))

```
error[E0603]: module `private` is private
  --> src/lib.rs:18:39
   |
18 |     fn type_id(&self, _: core::error::private::Internal) -> std::any::TypeId {
   |                                       ^^^^^^^ private module
   |
note: the module `private` is defined here
  --> /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/core/src/error.rs:206:1
```

Partially-sealed traits aren't limited to Rust's standard library. We can tweak our earlier `SealedTrait` to make it partially-sealed — we just need to provide a default implementation for *all* methods that take an argument that downstream crates cannot name.

```rust
mod private {
    pub struct Token;
}

pub trait PartiallySealedTrait {
    fn callable_method(&self);

    fn non_callable_method(&self, _: private::Token) {
        println!("you can't change this");
    }
}
```

Here you may have noticed that our trait's non-overridable method also cannot be *called* by downstream crates. However, we can allow downstream crates to call it *indirectly* by having our crate expose a function like: ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c34902cb5b96a325b2bd42a6aa7bcbdd))

```rust
pub fn call_method_indirectly(value: &PartiallySealedTrait) {
    value.non_callable_method(private::Token)
}
```

This is manageable, but hardly ergonomic. Fortunately, a better solution is on the horizon: [the "Final Trait Methods" pre-RFC](https://internals.rust-lang.org/t/pre-rfc-final-trait-methods/18407) seeks to add a `final` keyword or `#[final]` attribute to trait methods to prevent overriding *without* making them non-callable in downstream code.

## The full matrix of possibilities

The techniques described here let us choose precisely the combination between overridable and callable methods we wish to achieve in our traits:

|                                                              | all methods callable downstream                              | some methods callable downstream                             | no methods callable downstream   |
| ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | -------------------------------- |
| **all methods overridable**                                  | ✅ (`pub trait`)                                              | ❌                                                            | ❌                                |
| **some methods overridable**                                 | ✅ ([signature-sealed default methods + `pub fn` to call them](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=fcdedb4be8f688e36dc07e16df28c080) / ["final methods" pre-RFC](https://internals.rust-lang.org/t/pre-rfc-final-trait-methods/18407)) | ✅ (partially-sealed / ["final methods" pre-RFC](https://internals.rust-lang.org/t/pre-rfc-final-trait-methods/18407)) | ❌                                |
| **trait cannot be impl'd downstream (no methods overridable)** | ✅ (supertrait sealed)                                        | ✅ (at least one signature-sealed method with no default impl) | ✅ (all methods signature-sealed) |

I learned these techniques through reading obscure GitHub issues, Rust internals posts, and Zulip threads, and lots of painstaking experimentation.

I needed to be sure I understand the full menu of trait implementation options because [`cargo-semver-checks`](https://crates.io/crates/cargo-semver-checks) *must not* give incorrect advice. The role of linters is to take the author's expertise and make it generally available to everyone — and that required first acquiring some expertise in trait-sealing!

I've learned a great deal from thoughtful Rustaceans' blog posts, and I'm working to pay it forward! Hopefully this post saves future Rustaceans from needing to dig as deeply as I did 🦀
