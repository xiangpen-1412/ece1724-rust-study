# ECE1724 Rust 学习项目

保存自己写的课堂练习和学习笔记，使用 RustRover，通过 GitHub 在两台电脑之间同步。

仓库：[xiangpen-1412/ece1724-rust-study](https://github.com/xiangpen-1412/ece1724-rust-study)

SSH 地址：`git@github.com:xiangpen-1412/ece1724-rust-study.git`

## 项目结构

```text
src/main.rs               自己当前的练习入口
src/lectures/lec3/         Lecture 3：每个知识点一个独立练习文件
notes/                    自己的课堂笔记与练习记录
Cargo.toml                登记每个文件的运行目标
```

在 RustRover 中打开项目根目录。打开要练习的文件，点击 `fn main()` 旁的绿色运行按钮即可单独运行。

在 `src/lectures/lec3/` 新增文件后，需要在 `Cargo.toml` 中增加对应的 `[[bin]]`，格式参考已有条目。
每个文件使用自己的 `fn main()`，不用把所有知识点堆在一起。

## Lecture 3 练习

| 文件（位于 src/lectures/lec3/） | 运行目标 | 内容 |
| --- | --- | --- |
| `variables_and_mutability.rs` | `lec3_variables` | 变量与可变性 |
| `constants_and_static.rs` | `lec3_constants_static` | 常量与 static |
| `scope_and_shadowing.rs` | `lec3_scope_shadowing` | 作用域与 shadowing |
| `input_parsing.rs` | `lec3_input_parsing` | 输入、trim、parse |
| `type_inference.rs` | `lec3_type_inference` | 类型推断与类型约束 |
| `integer_types.rs` | `lec3_integer_types` | 整数范围与溢出 |
| `numeric_operations.rs` | `lec3_numeric_operations` | 浮点数、除法、余数与误差 |
| `type_conversions.rs` | `lec3_type_conversions` | 显式转换、窄化与转换时机 |
| `booleans.rs` | `lec3_booleans` | 条件类型与短路求值 |
| `characters.rs` | `lec3_characters` | char、字节与 UTF-8 长度 |

数据类型这六个文件包含简短的基础示例，扩展题以英文 TODO 注释列出，留给自己实现。所有代码注释统一使用英文。
先在聊天里学知识与推理，再打开对应文件：预测结果、写代码、运行验证、记录错因。
故意制造编译错误的实验，观察完后先注释掉，方便继续运行其他练习。

本轮练习索引和综合题：[Lecture 3 数据类型笔记](notes/lecture03-data-types.md)。

## 两台电脑同步

**开始学习前 Pull，结束学习后 Commit and Push。** 保存文件本身不会上传到 GitHub。

在 RustRover 中：

1. 开始前，执行 Git → Pull，取回另一台电脑最新的提交。
2. 写代码、保存并运行自己的练习。
3. 打开 Commit，检查并勾选本次修改的代码和笔记。
4. 写简短说明，例如“Lecture 3：练习 shadowing”。
5. 执行 Commit and Push，把记录上传到 GitHub。

尽量在一台电脑完成 Push 后，再切换到另一台电脑 Pull 并开始修改。
若 Pull 被拒绝或出现冲突，先保留本地修改再处理冲突，不要强制推送或直接覆盖。

### 第二台电脑首次使用

在 RustRover 中选择 Clone Repository / Get from Version Control，粘贴本页的 SSH 地址。
第二台电脑需要有你 GitHub 账户认可的 SSH 密钥。也可以使用 HTTPS 地址并登录账户：

`https://github.com/xiangpen-1412/ece1724-rust-study.git`

打开克隆得到的项目。此后使用 Pull / Commit and Push 同步，不再手工复制文件夹。

## Git Bash 常用操作

在项目文件夹中打开 Git Bash。

```bash
# Before studying
git pull --ff-only

# After studying: review the files to be committed
git status
git diff
git add src notes
git diff --cached
git commit -m "Lecture 3: practice notes"
git push
```

如果修改了 Cargo.toml、Cargo.lock 或其他配置，也需要把相应文件加入提交。

## Rust 环境与运行

项目使用 Rust 1.98.1、Rust 2024 edition，版本由 `rust-toolchain.toml` 固定。
另一台电脑需要安装 Rust 和对应操作系统的链接工具；Windows 使用原生 MSVC C++ Build Tools。

`Cargo.lock` 跟随项目提交；本机生成的 `target/`、`.idea/` 不上传。

```bash
# Run src/main.rs
cargo run --bin ece1724-rust-study

# Run the type inference example; use the table above for other targets
cargo run --bin lec3_type_inference

# Check that all exercises compile
cargo check --all-targets

# Check code formatting
cargo fmt --check
```

研究整数溢出时，可对同一个运行目标再加 `--release` 比较当前项目两种构建配置的行为。
