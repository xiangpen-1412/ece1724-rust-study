# RustLab

长期使用的 Rust 学习工程。用 RustRover 打开本目录的 `Cargo.toml`，全部旧代码和新练习都在这一个项目中。

## 当前学习

[Lecture 7：Vectors and Hash Maps](src/09-Vectors-HashMaps/README.md) 的跟课框架已就绪。打开 `src/09-Vectors-HashMaps/01_vectors.rs` 开始写；每个文件都有独立入口，点击 `fn main()` 左侧运行按钮，或在运行配置中选择对应的 `collections_*`。

```bash
cargo run
cargo run --bin collections_vector_access
cargo run --bin collections_hashmaps
cargo run --bin collections_word_count
```

直接 `cargo run` 默认进入 `collections_vectors`。本模块不需要额外依赖或外部数据文件。

[Lecture 6：错误处理](src/08-Error-Handling/README.md) 仍通过 `errors_*` 入口运行，例如 `cargo run --bin errors_question_mark`。Struct 练习入口仍为 `practice_structs`，原 `src/main.rs` 入口仍为 `scratch`。

## 知识模块

- [基础语法与数据类型](src/01-Rust-Basics/README.md)
- [函数与控制流](src/02-Functions-Control-Flow/README.md)
- [Ownership 与 Borrowing](src/03-Ownership-Borrowing/README.md)
- [String 与 Slice](src/04-Strings-Slices/README.md)
- [Struct、方法与关联函数](src/05-Structs-Methods/README.md)
- [Enum、Option 与模式匹配](src/06-Enums-Pattern-Matching/README.md)
- [OOP 与类型设计](src/07-OOP-Design/README.md)
- [错误处理](src/08-Error-Handling/README.md)
- [Vec 与 HashMap](src/09-Vectors-HashMaps/README.md)

## 原代码保全

迁移读取的是旧工程当前磁盘内容，包含未提交的 main.rs 与新增的 9 个文件。原工程 44 个 Rust 文件和 Tutorial 中 7 个练习文件全部按字节复制，未格式化、改注释或自动修复。只调整目录、Cargo 运行入口及项目名称。

旧路径、新路径、运行目标和 SHA-256 对照见 [migration-map.json](migration-map.json)。旧工程仍保留在原位置作为回退；以后在 RustLab 继续编辑。原有 Git 提交历史随项目保留，代码仓库沿用 [ece1724-rust-study](https://github.com/xiangpen-1412/ece1724-rust-study)。

项目固定 Rust 1.98.1 / edition 2024；保留原 log 依赖及其锁定版本。各示例中的既有 warning 不影响编译；有些示例故意演示 panic 或需要标准输入，按文件内容分别运行。

## 笔记与同步

长期笔记放在独立的 [Rust_Tutorial](https://github.com/xiangpen-1412/Rust_Tutorial)，本机位置为 `/Users/xiangpengji/Documents/Obsidian/Rust_Tutorial`。笔记按 Basics、Data Structures、Memory、Type System、OOP 分层，目录和文件均按讲义顺序编号；每篇概念笔记链接到本工程对应代码。

另一台电脑克隆代码仓库时，文件夹命名为 RustLab 即可：

```bash
git clone git@github.com:xiangpen-1412/ece1724-rust-study.git RustLab
```

开始学习前 Pull，完成后检查改动并 Commit and Push。笔记仓库和代码仓库分别同步；保存文件本身不会上传。
