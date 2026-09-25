# Lecture 3：循环

对应讲义第 41–43 页。代码只保留少量英文注释，详细规则放在本页。

| 文件 | 运行目标 | 重点 |
| --- | --- | --- |
| `loop_expressions.rs` | `lec3_loop_expressions` | 无条件循环、break 返回值、unit |
| `while_loops.rs` | `lec3_while_loops` | 先检查条件、状态更新、continue |
| `for_loops.rs` | `lec3_for_loops` | 遍历、范围、倒序、索引、嵌套循环标签 |

## 三种循环

- `loop`：一直重复，直到 break、return 等控制流使它退出。`break value` 可以给它提供结果。
- `while condition`：每轮先检查 bool 条件，因此可能一次都不执行。
- `for item in collection`：依次取出迭代产生的元素，通常不需要自己管理下标。

普通 while 和 for 不能直接用 `break value` 返回一个结果；它们可以使用没有值的 `break;`。
同一个 loop 的不同 break 路径，要提供兼容类型的结果。`break;` 提供 unit，不能与普通整数结果混用。
loop 内部最后写一个表达式，不会自动退出；需要使用 break。

## break、continue、return

| 写法 | 效果 |
| --- | --- |
| `break;` | 退出默认的最内层循环 |
| `continue;` | 跳过本轮剩余代码，进入下一轮 |
| `return value;` | 结束整个函数 |
| `break 'rows;` | 退出名为 rows 的外层循环 |

while 的 continue 会回到条件检查；for 的 continue 会继续取下一个元素。
如果 while 用计数器控制循环，continue 不能跳过必要的计数器更新，否则可能卡在同一个状态。

## 范围与数组边界

| 写法 | 产生的整数 |
| --- | --- |
| `1..4` | 1、2、3 |
| `1..=4` | 1、2、3、4 |
| `(1..4).rev()` | 3、2、1 |
| `4..1` | 没有元素，不是倒序 |
| `0..values.len()` | 数组的合法下标范围 |

不要把数组下标范围写成 `0..=values.len()`，因为它多包含一个越界下标。
空数组的 `0..values.len()` 自然为空，不需要先计算 `len() - 1`，也就不会引入无符号减法下溢。

只需要元素时直接遍历；同时需要下标和元素时用 `values.iter().enumerate()`。
这里的 iter 借用元素，value 是引用；借用的完整规则放在所有权章节学习。

当前 `for value in values` 的示例使用 i32 数组，元素可复制，因此随后还可以使用原数组。对于包含 String 等非 Copy 元素的集合，不能直接套用这个结论。
`for mut value in values` 修改的是当前绑定，不会自动修改数组里的元素。

## 新示例的预期输出

- loop：counter = 10、result = 20；第二个循环结束后 counter = 0、结果为 unit。
- while：倒数 3、2、1；初始条件为 false 时执行零次；1 到 5 的奇数和为 9。
- for：数组和为 60；范围与倒序结果见上表；逐个副本加一后，原数组仍为 [10, 20, 30]。
- 嵌套循环在 (1, 1) 处退出外层，此前访问了 (0, 0)、(0, 1)、(0, 2)、(1, 0)，共四次。
- first_even：在 2 处 return，整个函数直接返回 2。

## 两道练习

1. 用 loop 和带值的 break，找到第一个大于 20 的 7 的倍数。
2. 用 enumerate，找出非空数组 [12, 7, 19, 5] 的最大值及其下标，保存在一个元组里。

```bash
cargo run --bin lec3_loop_expressions
cargo run --bin lec3_while_loops
cargo run --bin lec3_for_loops
```

至此 Lecture 3 的正式知识点已经覆盖。第 44 页要求阅读 Rust Book 第 3 章。

来源：[Lecture 3](https://iqua.ece.toronto.edu/baochun/ece1724/slides/lecture3.html)、[Rust Book — Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)、[Rust Reference — Loop expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html)。
