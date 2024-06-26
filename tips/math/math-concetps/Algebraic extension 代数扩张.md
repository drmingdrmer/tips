- https://en.wikipedia.org/wiki/Algebraic_extension
- ### 定义
  
  一个域 \( E \) 对另一个域 \( F \) 的**扩张**（也称为**扩域**或**域扩张**），记作 \( E/F \)，是指 \( F \) 是 \( E \) 的一个子集，并且 \( E \) 中的加法和乘法运算限制到 \( F \) 上时，\( F \) 本身也构成一个域。
  
  当 \( E \) 中的每一个元素都是 \( F \) 中某个多项式的根时，这种扩张被称为**代数扩张**。换句话说，如果对于每个 \( \alpha \in E \)，都存在一个非零多项式 \( f(x) \) 属于 \( F[x] \)（即其系数在 \( F \) 中的多项式），使得 \( f(\alpha) = 0 \)，那么 \( E \) 对 \( F \) 的扩张称为代数扩张。
- ### 示例
  考虑域 \( \mathbb{Q} \)（有理数域）和域 \( \mathbb{Q}(\sqrt{2}) \)（包含所有形式为 \( a + b\sqrt{2} \) 的数，其中 \( a, b \) 是有理数）。这里，\( \mathbb{Q}(\sqrt{2}) \) 是 \( \mathbb{Q} \) 的一个代数扩张，因为 \( \sqrt{2} \) 是多项式 \( x^2 - 2 \) 的根，而这个多项式的系数（1和-2）都在 \( \mathbb{Q} \) 中。
- ### 可分与不可分的代数扩张
  
  在研究代数扩张时，一个重要的方面是扩张的**可分性**。如果 \( E/F \) 的每个元素的最小多项式在 \( F \) 上都没有重根，那么这个扩张被称为**可分扩张**: [[Separable extension 可分扩张(field)]]。可分性在数域理论和代数几何中特别重要，因为它关系到扩域的“行为”以及与之相关的结构和定理。
-
-
- ### 超越扩张
  在超越扩张中，扩大的域包含至少一个元素，这个元素不是基域中任何非零多项式的根。这些元素被称为超越元素。
- ### 定义
  超越扩张是指如果存在一个或多个元素 \( \alpha \in E \)（其中 \( E \) 是扩域），使得 \( \alpha \) 不是任何属于 \( F[x] \) 的非零多项式 \( f(x) \) 的根，那么域 \( E \) 相对于 \( F \) 的扩张是超越的。换句话说，这样的元素 \( \alpha \) 被称为超越元素，因为它们超越了代数关系的限制。
- ### 示例
  
  **实数域对有理数域的扩张包含超越扩张**：
- 例如，\( \pi \) 和 \( e \)（自然对数的底数）都是超越数。这意味着它们不是有理数域 \( \mathbb{Q} \) 上任何非零多项式的根。因此，域 \( \mathbb{Q}(\pi) \) 和 \( \mathbb{Q}(e) \) 是 \( \mathbb{Q} \) 的超越扩张。
  
  **函数域的扩张**：
- 另一个例子是考虑有理函数域 \( F(x) \)，其中 \( x \) 是一个超越元素。这个域包含了所有形式为 \( \frac{p(x)}{q(x)} \) 的元素，其中 \( p(x) \) 和 \( q(x) \) 是多项式且 \( q(x) \neq 0 \)。在这里，\( x \) 被视为一个不满足任何有理数系数多项式方程的变量，因此 \( F(x) \) 对 \( F \) 的扩展是超越的。