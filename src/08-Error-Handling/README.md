# Error Handling（错误处理）

按课程网站的讲义编号，本主题为 Lecture 6，接在 Structs and Enums 后面。官网列出的主题是可恢复与不可恢复错误、`panic!`、`Result`，以及何时选择 panic。以下文件是跟课用的空白入口，细分顺序便于练习，不表示老师一定按这些文件顺序讲。

- [课程主题](https://iqua.ece.toronto.edu/baochun/ece1724/lectures)
- [课程日期与 Quiz 安排](https://iqua.ece.toronto.edu/baochun/ece1724/)

## 写代码与运行

从 `01_panic.rs` 开始。每个文件都已有 `fn main()`，可以独立写代码；Cargo 已注册全部入口，不需要手动加模块声明。

在 RustRover 中点击 `fn main()` 左侧绿色运行按钮，或选择项目 `.run/` 中的 `errors_*` 运行配置。也可在 RustLab 根目录运行：

```bash
cargo run
cargo run --bin errors_result
cargo run --bin errors_question_mark
```

`cargo run` 默认打开本模块第一个入口 `errors_panic`。初始入口可以直接运行；错误处理代码留给你自己写。原有知识模块依然使用各自的运行目标。

| 文件 | 运行目标 | 主题 |
| --- | --- | --- |
| [01_panic.rs](01_panic.rs) | `errors_panic` | panic! and unrecoverable errors |
| [02_result.rs](02_result.rs) | `errors_result` | Result, Ok, and Err |
| [03_match_errors.rs](03_match_errors.rs) | `errors_match` | Handling errors with match |
| [04_unwrap_expect.rs](04_unwrap_expect.rs) | `errors_unwrap_expect` | unwrap and expect |
| [05_error_propagation.rs](05_error_propagation.rs) | `errors_propagation` | Returning errors to the caller |
| [06_question_mark.rs](06_question_mark.rs) | `errors_question_mark` | The ? operator |
| [07_panic_or_result.rs](07_panic_or_result.rs) | `errors_panic_or_result` | Choosing panic! or Result |

## 文件路径

运行配置的工作目录统一为 RustLab 项目根目录。涉及文件读取的入口已给出两个路径变量：

- `sample_path`：`src/08-Error-Handling/data/hello.txt`，文件已存在。
- `missing_path`：`src/08-Error-Handling/data/missing.txt`，故意不创建，用于练习读取失败。

在终端也应先进入 RustLab 根目录，再运行上述命令。所有新文件只使用标准库，无须添加依赖。
