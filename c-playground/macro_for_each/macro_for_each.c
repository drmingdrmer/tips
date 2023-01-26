// Implement for-each loop with preprocessor
// Usage:
//  gcc -E macro_for_each.c

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

// --- Example:

#define handle_error(m) if (m == err) {                                   \
    log_error("error is: %s", #m);                                        \
    return -1;                                                            \
}

for_each(handle_error, (a), (b), (c));

// --- Playground, ignore it:

int foo() {
    /*
     * #   define elts (a), (b), (c)
     * #   define handle_error(m) if (m == err) {                                    \
     *         log_error("error is: %s", #m);                                        \
     *         return -1;                                                            \
     *     }
     *
     *     for_each(handle_error, elts);
     */

    /*
     *     int x = arg1();
     *     x = IIF(BOOL(IS_PAREN(())))(a,b);
     *     x = is_empty(x);
     *     x = is_empty();
     *
     *     x = CHECK(1);
     *     x = CHECK(yy);
     *     x = CHECK();
     *     x = CHECK(PROBE(~));
     */

    /* x = WHEN(IS_PAREN(a))(1,2); */

    /*
     * for_each(printf,
     *      (a, 1),
     *      (a, 1),
     *      (a, 1),
     *      (a, 1),
     *      (b),
     *      (c)
     * );
     */

    nr = no_args(1,2,3,4);
}
