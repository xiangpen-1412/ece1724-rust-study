# Lecture 3：元组与数组

对应讲义第 28–35 页。聊天中的讲解配合两个完整示例文件复习：

| 文件 | 运行目标 | 内容 |
| --- | --- | --- |
| `src/lectures/lec3/11_tuples.rs` | `lec3_tuples` | 元组类型、解构、字段访问、可变性、单元素元组、unit、返回多个值 |
| `src/lectures/lec3/12_arrays.rs` | `lec3_arrays` | 数组类型、重复初始化、修改元素、usize 索引、边界检查、二维数组 |

## 查阅要点

| 写法 | 含义或结果 |
| --- | --- |
| `(120, 2.5, 3_u8)` | 一个元组，三个位置可以分别有不同类型 |
| `let (count, price, level) = record;` | 按位置解构，并建立三个新绑定 |
| `record.0` | 元组第一个字段，字段号是固定的 |
| `(42)` / `(42,)` | 前者是整数；后者是单元素元组 |
| `()` | unit 类型及其唯一的值，不能理解成“不存在任何值的类型” |
| `{ 7 }` / `{ 7; }` | 前者得到 7，后者得到 unit |
| `[i32; 4]` | 四个 i32 的数组类型，长度是类型的一部分 |
| `[3, 5]` / `[3; 5]` | 前者两个元素，后者五个 3 |
| `values[index]` | index 是 usize；运行时越界会 panic |
| `values.get(index)` | 有元素得到 Some，没有元素得到 None，不因越界 panic |
| `[[i32; 3]; 2]` | 两行，每行三个 i32 |

`mut` 允许修改值，不会放宽类型限制，也不会让数组增长。解构出来的新绑定不会自动继承原绑定的可变性。

当前元组示例的字段都是实现 Copy 的数值类型，所以解构后原元组仍可使用。不要把这个结论直接推广到含 String 的元组；所有权部分再展开。

数组的元素连续存储，但数组类型本身不保证总在栈上。固定大小和实际存储位置是两个问题。

数组越界的检查也不等于“编译器永远发现不了”：静态可确定的越界通常会被诊断；运行时才确定的非法索引会 panic。release 构建也不会允许安全索引任意越界。

## 运行与观察

```bash
# Run the complete reference examples
cargo run --bin lec3_tuples
cargo run --bin lec3_arrays

# Observe checked access using a runtime index
cargo run --bin lec3_arrays -- 2
cargo run --bin lec3_arrays -- 4

# Deliberately observe an out-of-bounds panic
cargo run --bin lec3_arrays -- 4 --panic
```

`Some`、`None` 属于后面要系统学习的 Option。现在只需理解 get 的两个可能结果。数组文件中的命令行处理也已经完整写好，不需要先掌握其中所有语法才能运行核心例子。

## 只留两道动手题

1. 在 `11_tuples.rs` 结尾完成坐标解构与 shadowing：先预测，再验证旧绑定和新元组分别是什么。
2. 在 `12_arrays.rs` 结尾完成二维数组的安全写入：分别尝试合法与非法坐标，写入之前检查两个边界。

其余知识点都已有完整代码和说明，可以随时查看。

课程来源：[Lecture 3](https://iqua.ece.toronto.edu/baochun/ece1724/slides/lecture3.html)。
规则参考：[Rust Book — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)、[Tuple expressions](https://doc.rust-lang.org/reference/expressions/tuple-expr.html)、[Array](https://doc.rust-lang.org/std/primitive.array.html)。
