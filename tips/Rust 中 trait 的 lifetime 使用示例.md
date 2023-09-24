tags:: tips, rust-programming, trait,  rust-lifetime

Rust 中 trait 和 lifetime 结合使用会让编程变得复杂.
因为多个trait 对多个 struct 的实现之间会产生 lifetime 参数的影响.
例如某个 trait 的定义可以实现给A, B两个struct, 但再实现给C就会报错.

举个例子, 假设我们要设计一个简单的KV接口的MapApi:


-   [map-api-simplified.rs](../rust-playground/src/bin/map-api-simplified.rs)

    我们给出一个不含有 lifetime 参数简化版的设计,
    及实现了Level, Writable, StaticLevels, RefMut 这4个 struct,
    用来展示功能.

    这个版本的map trait实现给'static 类型的数据, 每个trait method 接受一个引用.
    所以这个map trait 无法为一个临时变量实现, 例如下面的代码中,
    tmp是一个临时变量, 则MapApi::get(&self) 在这个场合就无法使用了.

    ```rust
    fn foo() -> () {
        let tmp = Tmp::new();
        tmp.get("key").await
    }
    ```

-   [map-api-lifetime.rs](../rust-playground/src/bin/map-api-lifetime.rs)

    然后给出一个带 lifetime 参数的trait设计也实现了以上4个 struct,
    用来展示复杂的 lifetime trait 如何使用,
    这个例子里 trait 设计为可以实现给非 `'static` 的对象使用, 提供了更多灵活性,
    但也给 `trait` 设计带来的挑战.
    在这个实现中有2个重要的 `lifetime` 参数为 `'me` 和 `'d`,
    用于描述 Self 的 lifetime 和它引用(或持有)的数据的 lifetime;
    例如`&'me mut RefMut<'d> { data: &'d mut Level }` 中 Self 就允许有不同的 lifetime:


另外2个例子展示了不当的设计产生的问题:

-   [map-api-error-map-api-need-2-param.rs](../rust-playground/src/bin/map-api-error-map-api-need-2-param.rs)

    MapApi 需要 2个 lifetime 参数: `'me` 和 `'d`,
    用于描述 Self 的 lifetime 和它引用(或持有)的数据的 lifetime;
    例如`&'me 'mut RefMut<'d> { data: &'d mut Level }`, 虽然大部分时候, `'me` 和 `'d` 是一样的, 这个例子展示了为何必须区分这2个 lifetime:
    `Error: cannot infer an appropriate lifetime for lifetime parameter `'d` due to conflicting requirements`:

-   [map-api-error-2-lifetime.rs](../rust-playground/src/bin/map-api-error-2-lifetime.rs)

    `async move {}` block 生成的 Future 中如果 ref 了其他数据,
    这个 Future 则包含一个 lifetime, 如果 ref 了多个数据, 会产生 `lifetime used multiple times` 的错误

    ```
    note: lifetime used multiple times
       --> src/bin/map-api-error-2-lifetime.rs:175:6
        |
    175 | impl<'me, 'd, K> MapApiRO<'d, K> for &'me Writable<'d>
        |      ^^^^^^  ^^^^^
    ```

    解决方法是把多个 ref 的 lifetime 归到一个新的lifetime参数`'f` 上:

    ```rust
    impl<'me, 'd, K> MapApiRO<'d, K> for &'me Writable<'d>
    where
        K: MapKey,
        for<'him> &'him Level: MapApiRO<'him, K>,
    {
        type GetFut<'f, Q> = impl Future<Output =K::V> + 'f
        where Self: 'f,
              'me: 'f,
              'd: 'f,
              K: Borrow<Q>,
              Q: Ord + Send + Sync + ?Sized,
              Q: 'f;

        fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
        where
            'me: 'f,
            'd: 'f,
            K: Borrow<Q>,
            Q: Ord + Send + Sync + ?Sized,
        {
            async move {
                let level_data = &*self.writable;
                let got = level_data.get(key).await;
                got
            }
        }
    }
    ```

    TODO: 或者为GAT配置多个lifetime参数?



