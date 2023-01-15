title:: 存储一个 binary-tree 所需的最小空间
type:: [[Tips]]
tags:: math, math-combinatorics, catalan-number, binary-tree

**应用**: 既然n个节点的 binary-tree 的数量是确定的, 那么我们就可以给每个tree编号,
并通过存储编号来存储一个tree.
且编号所需的bit数, 就是存储一个 binary-tree 的理论上最小的空间开销.

例如上面3节点binary-tree, 有5个不同的树, 所以存储3节点binary-tree最少需要3个bit

对于任意n, 存储n节点的binary-tree, 最少需要的bit数是

$$
log_2 C_n = log_2 ((2n)!) - 2 log_2 (n!) - log_2 (n+1)
$$

通过 [Stirling Approximation](https://en.wikipedia.org/wiki/Stirling%27s_approximation), 近似阶乘的计算:

$$
log_2 C_n = 2n + o(n)
$$


也就是说我们用2n个bit就可以存储任意一个n节点的binary-tree

编码一个binary-tree的方法也很简单: 
```
fn encode(output: &mut Vec<bit>, node) {
    if node.is_none() {
        output.push(0);
    } else {
        output.push(1);
        encode(output, node.left);
        encode(output, node.rigth);
    }
}
```

`(x*x)*(x*x)`: `1 (1 (0) (0)) (1 (0) (0))`
