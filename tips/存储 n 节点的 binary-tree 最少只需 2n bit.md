tags:: tips, math, math-combinatorics, catalan-number, binary-tree


**原理**:
[n 节点 binary-tree 的个数](./n%20节点%20binary-tree%20的个数.md) 有 $C_n = \frac{1}{n+1} {2n \choose n}$ 个,
那么最节省空间的存储 binary-tree 的方法就是:
**枚举所有 n 个节点的 binary-tree, 然后只存储 binary-tree 的编号**.
因此表示编号所需的 bit 数, 就是存储一个 binary-tree 的理论上最小的空间开销, 即:

$$
log_2 C_n = log_2 ((2n)!) - 2 log_2 (n!) - log_2 (n+1)
$$

通过 [Stirling Approximation](https://en.wikipedia.org/wiki/Stirling%27s_approximation), 近似阶乘的计算后:

$$
log_2 C_n = 2n + o(n)
$$


这表示理论上我们用 `2n` 个 bit 就可以存储任意一个 n 节点的 binary-tree.

**实现**: 要达到这样的空间开销, 一种实际可行的方法是按照 `node, node.left_child, node.right_child` 的顺序将节点编码到一个`2n+1` bit 的数组中:

```
fn encode(output: &mut Vec<bit>, node) {
    if node.is_null() {
        output.push(0);
    } else {
        output.push(1);
        encode(output, node.left);
        encode(output, node.rigth);
    }
}
```

例如用`*` 表示树的节点, `x`表示null, 3节点 binary-tree 被编码为长度为7的bit数组(忽略括号):

```
   *
  / \
 *   *     --->  1 (1 (0) (0)) (1 (0) (0))
/ \ / \
x x x x

   *
  / \
 *   x     --->  1 (1 (0) (1 (0) (0))) (0)
/ \
x  *
  / \
  x x
```


<!--

```
C₀C₂    00..    000.
        010.    001.
        011.    01..
        1...    1...


C₁C₁    00..
        01..
        10..
        11..


C₂C₀    0...    0...
        100.    10..
        101.    110.
        11..    111.


```
-->

<!--

```
C₀C₂       0  0  .  .  .             0  0  0  .  .
           0  1  0  .  .             0  0  1  .  .
           0  1  1  .  .             0  1  .  .  .
           1  .  .  .  .             1  .  .  .  .


C₁C₁       0  0  .  .  .
           0  1  .  .  .
           1  0  .  .  .
           1  1  .  .  .


C₂C₀       0  .  .  .  .             0  .  .  .  .
           1  0  0  .  .             1  0  .  .  .
           1  0  1  .  .             1  1  0  .  .
           1  1  .  .  .             1  1  1  .  .
```

-->
