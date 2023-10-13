tags:: tips, rust-programming, type, variance, rust-lifetime

GPT翻译自:
https://doc.rust-lang.org/nomicon/subtyping.html

摘自于线上书:
`<< The Rustonomicon >>`: The Dark Arts of Advanced and Unsafe Rust Programming
https://doc.rust-lang.org/nomicon/intro.html
https://github.com/rust-lang/nomicon

另一个关于 subtyping and variance 的解释:
https://doc.rust-lang.org/reference/subtyping.html#variance

---


# Subtyping(子类型) and Variance(变型)

Rust 使用生命周期来追踪借用与所有权之间的关系。然而，对生命周期的朴素实现可能过于严格，或者允许未定义行为。

为了允许灵活地使用生命周期，同时又防止他们的误用，Rust 使用子类型和变型。

让我们以一个例子开始。

```rust
// 注意：debug 需要两个参数具有*相同*的生命周期
fn debug<'a>(a: &'a str, b: &'a str) {
    println!("a = {a:?} b = {b:?}");
}

fn main() {
    let hello: &'static str = "hello";
    {
        let world = String::from("world");
        let world = &world; // 'world 的生命周期比 'static 短
        debug(hello, world);
    }
}
```

在对生命周期的保守实现中，由于 `hello` 和 `world` 有不同的生命周期，我们可能会看到以下错误：

```text
error[E0308]: mismatched types
 --> src/main.rs:10:16
   |
10 |         debug(hello, world);
   |                      ^
   |                      |
   |                      expected `&'static str`, found struct `&'world str`
```

这将非常不幸。在这种情况下，我们希望接受任何生存期至少和 'world 一样长的类型。让我们尝试使用我们的生命周期子类型。

## 子类型

子类型是一种类型可以用于替代另一种类型的概念。

定义 `Sub` 是 `Super` 的子类型（我们将在本章中使用符号 `Sub <: Super`）。

这向我们表明 `Super` 定义的一组*要求*完全由 `Sub` 满足。 `Sub` 可能有更多的要求。

现在，为了使用生命周期的子类型，我们需要定义生命周期的要求：

> `'a` 定义了一段代码。

现在我们已经为生命周期定义了一组要求，我们可以定义它们彼此之间的关系：

> `'long <: 'short` 当且仅当 `'long` 定义了完全包含 `'short` 的代码区域。

`'long` 可能定义了比 `'short` 更大的区域，但仍符合我们的定义。

> 正如我们将在本章的其余部分看到的，子类型比这更复杂和微妙，但这个简单的规则是一个非常好的 99% 的直觉。除非你写不安全的代码，否则编译器会自动处理所有的角落情况。

> 但这是 Rustonomicon。我们正在编写不安全的代码，所以我们需要理解这个东西是如何真正工作的，以及我们如何可能搞砸它。

回到我们上面的例子，我们可以说 `'static <: 'world`。现在，让我们也接受这个想法，生命周期的子类型可以通过引用传递（更多关于这个在 [Variance](https://doc.rust-lang.org/nomicon/subtyping.html#variance)），例如 `&'static str` 是 `&'world str` 的子类型，然后我们可以 “降级” `&'static str` 成为一个 `&'world str`。有了这个，上面的例子就可以编译了：

```rust
fn debug<'a>(a: &'a str, b: &'a str) {
    println!("a = {a:?} b = {b:?}");
}

fn main() {
    let hello: &'static str = "hello";
    {
        let world = String::from("world");
        let world = &world; // 'world 的生命周期比 'static 短
        debug(hello, world); // hello 无声地从 `&'static str` 降级为 `&'world str`
    }
}
```

## 变型

在上面，我们简单地介绍了 `'static <: 'b` 意味着 `&'static T <: &'b T`。这使用了被称为*变型*的属性。然而，这并不总是像这个例子那么简单。为了理解这个，让我们试着扩展一下这个例子：

```rust
fn assign<T>(input: &mut T, val: T) {
    *input = val;
}

fn main() {
    let mut hello: &'static str = "hello";
    {
        let world = String::from("world");
        assign(&mut hello, &world);
    }
    println!("{hello}"); // 使用后释放 😿
}
```

在 `assign` 中，我们正在设置 `hello` 引用指向 `world`。但是 `world` 在 `println` 后面的 `hello` 使用之前就已经超出了作用域！

这是一个经典的使用后释放错误！

我们的第一反应可能是责怪 `assign` 实现，但是这里真的没有什么问题。我们可能希望将一个 `T` 分配给一个 `T`。

问题在于我们不能假设 `&mut &'static str` 和 `&mut &'b str` 是兼容的。这意味着 `&mut &'static str` **不能**是 `&mut &'b str` 的*子类型*，即使 `'static` 是 `'b` 的子类型。

变型是 Rust 借用来定义子类型通过他们的泛型参数的关系的概念。

> 注意：为了方便，我们将定义一个泛型类型 `F<T>`，这样我们就可以轻松地谈论 `T`。希望这在上下文中是清楚的。

类型 `F` 的*变型*是其输入的子类型如何影响其输出的子类型。在 Rust 中，有三种变型。给定两种类型 `Sub` 和 `Super`，其中 `Sub` 是 `Super` 的子类型：

- 如果 `F<Sub>` 是 `F<Super>` 的子类型（子类型属性“通过”），则 `F` 在 `T` 上是**协变的**。
- 如果 `F<Super>` 是 `F<Sub>` 的子类型（子类型属性“反转”），则 `F` 在 `T` 上是**逆变的**。
- 否则，`F` 在 `T` 上是**不变的**（不存在可以导出的子类型关系）。

如果我们记住上述例子，那么如果 `'a <: 'b`，我们可以将 `&'a T` 视为 `&'b T` 的子类型，因此我们可以说 `&'a T` 在 `'a` 上是*协变的*。

另外，我们看到我们不能将 `&mut &'a U` 视为 `&mut &'b U` 的子类型，因此我们可以说 `&mut T` 在 `T` 上是*不变的*。

以下是一些其他泛型类型及其变型的表：

|                 |    'a     |         T         |     U     |
| --------------- | :-------: | :---------------: | :-------: |
| `&'a T`         | 协变      |     协变          |           |
| `&'a mut T`     | 协变      |     不变          |           |
| `Box<T>`        |           |     协变          |           |
| `Vec<T>`        |           |     协变          |           |
| `UnsafeCell<T>` |           |     不变          |           |
| `Cell<T>`       |           |     不变          |           |
| `fn(T) -> U`    |           | **逆变**          | 协变      |
| `*const T`      |           |     协变          |           |
| `*mut T`        |           |     不变          |           |

其中一些可以简单地解释为与其他类型的关系：

- `Vec<T>` 和所有其他所有权指针和集合遵循与 `Box<T>` 相同的逻辑
- `Cell<T>` 和所有其他内部可变性类型遵循与 `UnsafeCell<T>` 相同的逻辑
- `UnsafeCell<T>` 具有内部可变性，使其具有与 `&mut T` 相同的变型属性
- `*const T` 遵循 `&T` 的逻辑
- `*mut T` 遵循 `&mut T`（或 `UnsafeCell<T>`）的逻辑

有关更多类型，请参见参考文档的 ["Variance" 部分](https://doc.rust-lang.org/reference/subtyping.html#variance)。

> 注意：语言中唯一的逆变来源是函数的参数，这就是为什么在实践中它并不常见。调用逆变涉及到使用特定生命周期的引用的高阶编程（相对于通常的 "any lifetime"，这涉及到高等级生命周期，这些工作独立于子类型）。

现在我们对变型有了更正式的理解，让我们更详细地看一些更多的例子。

```rust
fn assign<T>(input: &mut T, val: T) {
    *input = val;
}

fn main() {
    let mut hello: &'static str = "hello";
    {
        let world = String::from("world");
        assign(&mut hello, &world);
    }
    println!("{hello}");
}
```

当我们运行这个时会得到什么？

```text
error[E0597]: `world` does not live long enough
  --> src/main.rs:9:28
   |
6  |     let mut hello: &'static str = "hello";
   |                    ------------ type annotation requires that `world` is borrowed for `'static`
...
9  |         assign(&mut hello, &world);
   |                            ^^^^^^ borrowed value does not live long enough
10 |     }
   |     - `world` dropped here while still borrowed
```

好的，它没有编译通过！让我们详细分解这里发生了什么。

首先让我们看看 `assign` 函数：

```rust
fn assign<T>(input: &mut T, val: T) {
    *input = val;
}
```

它所做的只是取一个可变引用和一个值，并用值覆盖引用。这个函数的重要之处在于它创建了一个类型等价约束。它在其签名中清楚地表示引用和值必须是*完全相同*的类型。

与此同时，我们在调用者中传入 `&mut &'static str` 和 `&'world str`。

因为 `&mut T` 在 `T` 上是不变的，编译器得出结论，它不能应用任何子类型化到第一个参数，因此 `T` 必须完全是 `&'static str`。

这与 `&T` 的情况相反：

```rust
fn debug<T: std::fmt::Debug>(a: T, b: T) {
    println!("a = {a:?} b = {b:?}");
}
```

在这里，类似地，`a` 和 `b` 必须有相同的类型 `T`。但是由于 `&'a T` 是 `'a` 的协变的，我们被允许进行子类型化。因此，编译器决定 `&'static str` 可以变为 `&'b str` 当且仅当 `&'static str` 是 `&'b str` 的子类型，这将在 `'static <: 'b` 的情况下保持。这是真的，所以编译器很乐意继续编译这段代码。

事实证明，为什么 Box（和 Vec，HashMap 等）可以是协变的的论点与为什么生命周期可以是协变的的论点非常相似：一旦你试图将它们塞入像可变引用这样的东西，它们就会继承不变性，并阻止你做任何坏事。

然而，Box 使得我们更容易关注我们部分忽略的引用的按值方面。

与许多允许随时自由别名的语言不同，Rust 有一个非常严格的规则：如果你被允许修改或移动一个值，你就保证是唯一能够访问它的人。

考虑以下代码：

```rust
let hello: Box<&'static str> = Box::new("hello");

let mut world: Box<&'b str>;
world = hello;
```

完全没有问题，我们已经忘记了 `hello` 是 `'static`，因为一旦我们将 `hello` 移动到只知道它是 `'b` 存活的变量，**我们就摧毁了宇宙中唯一记住它生存时间更长的东西**！

现在只剩下一个东西要解释：函数指针。

要理解为什么 `fn(T) -> U` 应该对 `U` 是协变的，考虑以下签名：

```rust
fn get_str() -> &'a str;
```

这个函数声称产生一个绑定了某个生存期 `'a` 的 `str`。因此，完全可以提供一个具有以下签名的函数：

```rust
fn get_static() -> &'static str;
```

因此，当函数被调用时，它所期望的只是一个 `&str`，其生存期至少为 `'a`，实际上生存的时间更长并不重要。

然而，相同的逻辑并不适用于*参数*。考虑试图满足：

```rust
fn store_ref(&'a str);
```

与：

```rust
fn store_static(&'static str);
```

第一个函数可以接受任何字符串引用，只要它至少为 `'a` 存活，但是第二个无法接受任何生存期少于 `'static` 的字符串引用，这将导致冲突。协变在这里不起作用。但是，如果我们反过来想，它实际上就会*起作用*！如果我们需要一个可以处理 `&'static str` 的函数，那么一个可以处理*任何*引用生存期的函数肯定会很好用。

让我们在实践中看看

```rust
thread_local! {
    pub static StaticVecs: RefCell<Vec<&'static str>> = RefCell::new(Vec::new());
}

/// 将给定的输入保存到线程本地 `Vec<&'static str>`
fn store(input: &'static str) {
    StaticVecs.with(|v| {
        v.borrow_mut().push(input);
    })
}

/// 用它的输入调用函数（必须有相同的生存期！）
fn demo<'a>(input: &'a str, f: fn(&'a str)) {
    f(input);
}

fn main() {
    demo("hello", store); // "hello" 是 'static。可以很好地调用 `store`

    {
        let smuggle = String::from("smuggle");

        // `&smuggle` 不是 static。如果我们用 `&smuggle` 调用 `store`，
        // 我们将在 `StaticVecs` 中推入一个无效的生存期。
        // 因此，`fn(&'static str)` 不能是 `fn(&'a str)` 的子类型
        demo(&smuggle, store);
    }

    StaticVecs.with(|v| {
        println!("{:?}", v.borrow()); // 使用后释放 😿
    });
}
```

这就是为什么函数类型，与语言中的任何其他东西不同，对它们的参数是**逆变**的。

现在，对于标准库提供的类型来说这一切都很好，但是你定义的类型的变型是如何确定的呢？一种非正式的说法是，结构体继承其字段的变型。如果结构体 `MyType` 在字段 `a` 中使用了泛型参数 `A`，那么 `MyType` 对 `A` 的变型就完全是 `a` 对 `A` 的变型。

然而，如果 `A` 在多个字段中使用：

- 如果 `A` 的所有使用都是协变的，那么`MyType` 在 `A` 上是协变的
- 如果 `A` 的所有使用都是逆变的，那么 `MyType` 在 `A` 上是逆变的
- 否则，`MyType` 在 `A` 上是不变的

```rust
use std::cell::Cell;

struct MyType<'a, 'b, A: 'a, B: 'b, C, D, E, F, G, H, In, Out, Mixed> {
    a: &'a A,     // 在 'a 和 A 上是协变的
    b: &'b mut B, // 在 'b 上是协变的，在 B 上是不变的

    c: *const C,  // 在 C 上是协变的
    d: *mut D,    // 在 D 上是不变的

    e: E,         // 在 E 上是协变的
    f: Vec<F>,    // 在 F 上是协变的
    g: Cell<G>,   // 在 G 上是不变的

    h1: H,        // 也将在 H 上是协变的，除非...
    h2: Cell<H>,  // 在 H 上是不变的，因为不变性在所有冲突中都赢

    i: fn(In) -> Out,       // 在 In 上是逆变的，在 Out 上是协变的

    k1: fn(Mixed) -> usize, // 将在 Mixed 上是逆变的，除非..
    k2: Mixed,              // 在 Mixed 上是不变的，因为不变性在所有冲突中都赢
}
```
