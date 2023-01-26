tags:: tips, c-programming, macro, preprocessor, loop, for

C 语言里如果需要对多个 error-code 实现类似的错误处理,
可以借助宏 [preprocessor](https://en.wikipedia.org/wiki/C_preprocessor) 
来实现重复代码的生成.

虽然 C 语音的宏,
只提供了简单的判断和函数的语法, 如
`#if VERBOSE >= 2`,
`#define RADTODEG(x) ((x) * 57.29578)`, 
没有类似 [Rust macro 的循环](https://doc.rust-lang.org/rust-by-example/macros/repeat.html) 支持,
我们也可以用递归的方式实现一个 [`for_each`](../c-playground/macro_for_each/macro_for_each.c) 的宏来生成重复代码. 

```c
// cat for_each.c:
#include "for_each.h"
#define handle_error(m) if (m == err) {                                   \
    log_error("error is: %s", #m);                                        \
    return -1;                                                            \
}

for_each(handle_error, (a), (b), (c));
```

使用 `gcc -E macro_for_each.c` 展开宏后输出生成的代码:

```c
if (a == err) { log_error("error is: %s", "a"); return -1; }
if (b == err) { log_error("error is: %s", "b"); return -1; }
if (c == err) { log_error("error is: %s", "c"); return -1; }
```

```c
// cat for_each.h
#define EVAL(...)  EVAL1(EVAL1(EVAL1(__VA_ARGS__)))
#define EVAL1(...) EVAL2(EVAL2(EVAL2(__VA_ARGS__)))
#define EVAL2(...) EVAL3(EVAL3(EVAL3(__VA_ARGS__)))
#define EVAL3(...) EVAL4(EVAL4(EVAL4(__VA_ARGS__)))
#define EVAL4(...) EVAL5(EVAL5(EVAL5(__VA_ARGS__)))
#define EVAL5(...) __VA_ARGS__

#define arg1(a, ...) a
#define shift(a, ...) __VA_ARGS__

#define __empty_ 1
#define arg3(a1, a2, a3, ...) a3
/* #define is_empty(ph, ...) arg3(ph, ##__VA_ARGS__, 1, 0) */
#define is_empty(ph, ...) ph, ##__VA_ARGS__, 1

#define empty()
#define defer(id) id empty()
#define obstruct(...) __VA_ARGS__ defer(empty)()

#define CAT(a, ...) PRIMITIVE_CAT(a, __VA_ARGS__)
#define PRIMITIVE_CAT(a, ...) a ## __VA_ARGS__

#define COMPL(b) PRIMITIVE_CAT(COMPL_, b)
#define COMPL_0 1
#define COMPL_1 0

#define IIF(cond) CAT(IIF_,cond)
#define IIF_0(t, f) f
#define IIF_1(t, f) t

#define NOT(x) CHECK(PRIMITIVE_CAT(NOT_, x))
#define NOT_0 PROBE(~)

#define BOOL(x) COMPL(NOT(x))
#define IF(c) IIF(BOOL(c))

#define CHECK_N(x, n, ...) n
#define CHECK(...) CHECK_N(__VA_ARGS__, 0,)
#define PROBE(x) x, 1,

#define EAT(...)
#define EXPAND(...) __VA_ARGS__
#define WHEN(c) IF(c)(EXPAND, EAT)

#define IS_PAREN(x) CHECK(IS_PAREN_PROBE x)
#define IS_PAREN_PROBE(...) PROBE(~)

#define no_args_x(...) #__VA_ARGS__
#define no_args(...) 0 EVAL(no_args_(__VA_ARGS__, ()))

#define no_args_(c, ...)                                                      \
        IIF(BOOL(IS_PAREN(c)))( ,                                             \
            + 1                                                               \
            obstruct(no_args_xx)()(__VA_ARGS__)                               \
        )

#define no_args_xx() no_args_

#define for_each(statement, ...) EVAL(foreach(statement, __VA_ARGS__, ~))

#define foreach(statement, c, ...)                                            \
        IIF(BOOL(IS_PAREN(c)))(                                               \
            statement c                                                       \
            obstruct(foreach_xx)()(statement, __VA_ARGS__)                    \
            ,                                                                 \
        )

#define foreach_xx() foreach
```
