# Lecture 3：函数与 if 表达式

对应讲义第 36–40 页。完整参考代码位于 `src/lectures/lec3/`，代码注释使用英文。每个新文件只留一道综合练习。

| 文件 | 运行目标 | 内容 |
| --- | --- | --- |
| `13_functions.rs` | `lec3_functions` | 函数签名、参数、返回类型、块表达式、分号、打印与返回、参数的局部可变性 |
| `14_if_expressions.rs` | `lec3_if_expressions` | 条件类型、分支顺序、if 的值、分支类型、提前返回、边界情况 |

## 阅读代码时的三个检查点

1. 函数签名：传入什么类型，正常返回时需要提供什么类型？
2. 每个块：正常执行到末尾时，最后产生什么类型的值？
3. 控制流：这一条路径继续执行，还是已经 return 离开整个函数？

普通函数的参数需要声明类型。省略 `->` 表示返回 `()`，不是让编译器根据函数体推断返回类型。函数在源码中可以放在调用者后面，只要处于可访问的作用域。

## 分号与返回值

| 函数体或表达式 | 含义 |
| --- | --- |
| `fn f() -> i32 { 8 }` | 尾表达式提供整数结果 |
| `fn f() -> i32 { return 8; }` | 显式返回整数，结束函数 |
| `fn f() -> i32 { 8; }` | 错误：正常到末尾时得到 unit，无法满足 i32 |
| `fn f() { 8 }` | 错误：默认返回 unit，不能用整数作为尾结果 |
| `fn f() { println!("8"); }` | 打印文字，返回 unit |
| `{ let n = 7; n + 1 }` | 得到整数 8 |
| `{ let n = 7; n + 1; }` | 得到 unit |

分号不会让 `return 8;` 失去返回作用：return 已经转移控制流。正常块的尾表达式和显式 return 是两种不同的返回写法。

赋值表达式的结果是 unit；把赋值放在末尾，也不会自动返回被赋进去的数值。

## if 的值和路径

```rust
let enabled = true;
let value = if enabled {
    10
} else {
    20
};
```

分支内的 10 和 20 是尾表达式；最后 `};` 的分号结束外层 let 语句。它不会把 value 变成 unit。

只有选中的分支在运行时执行，但未选中的普通分支也要通过类型检查。`if true { 10 } else { "twenty" }` 仍有类型错误。

正常产生值的分支需要兼容到一个确定的结果类型。这里不展开引用转换等进阶规则；整数与字符串、整数与 unit 不兼容，整数与浮点数之间也不会自动做数值转换。

如果某个分支执行 `return`，它直接退出函数，没有给 if 提供一个继续执行时使用的结果。因此它与“分支末尾得到 unit”不是一回事。

`else if` 只执行第一个满足条件的分支。范围较宽的条件放在前面，可能让后面更具体的条件失去作用。

## 讲义边界例子

`scale_small` 在 `-10 < number < 10` 时乘十，其余情况除以二。Rust 中要写成 `number > -10 && number < 10`，不能写数学式的连续比较。

| 输入 | 结果 | 原因 |
| --- | --- | --- |
| 9 | 90 | 在开区间内 |
| 10 | 5 | 边界不属于开区间 |
| -9 | -90 | 在开区间内 |
| -10 | -5 | 边界走除法 |
| -11 | -5 | 整数除法向零截断 |

`is_divisible_by` 先处理除数为零，再计算余数。本示例明确约定除数为零时返回 false；若忽略检查，整数 `% 0` 会 panic。

## 运行与复习

```bash
cargo run --bin lec3_functions
cargo run --bin lec3_if_expressions
```

先预测各条输出，再运行检查。遇到编译错误，先看 expected/found：它通常能直接指出哪条路径产生了错误的类型。

来源：[Lecture 3](https://iqua.ece.toronto.edu/baochun/ece1724/slides/lecture3.html)、[Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)、[Block expressions](https://doc.rust-lang.org/reference/expressions/block-expr.html)、[If expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html)。
