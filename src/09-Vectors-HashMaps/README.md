# Lecture 7 — Vectors and Hash Maps

[课程网站](https://iqua.ece.toronto.edu/baochun/ece1724/lectures) 的 Lecture 7 主题为 Vectors and Hash Maps，接在 Error Handling 后面。这里按知识点拆成 10 个可独立运行的跟课入口；编号是工程内的练习顺序。

目录编号 `09` 延续 RustLab 的知识模块编号，不代表 Lecture 9。

## 使用方式

打开 `01_vectors.rs` 开始写。每个文件已有 `fn main()`、英文提示和必要的已学知识准备，当前知识点的实现留空。

点击 RustRover 中 `fn main()` 左侧的运行按钮，或在运行配置里选择对应的 `collections_*`。Cargo 已注册全部入口，运行配置的工作目录为 RustLab 根目录。

在工程根目录也可以运行：

```bash
cargo run
cargo run --bin collections_vector_access
cargo run --bin collections_hashmaps
cargo run --bin collections_word_count
```

默认入口为 `collections_vectors`。本模块只使用标准库，无需新增依赖。

| 文件 | 运行目标 | 主题 |
| --- | --- | --- |
| [01_vectors.rs](01_vectors.rs) | `collections_vectors` | Creating vectors |
| [02_vector_updates.rs](02_vector_updates.rs) | `collections_vector_updates` | Adding and removing elements |
| [03_vector_access.rs](03_vector_access.rs) | `collections_vector_access` | Indexing and get |
| [04_vector_borrowing.rs](04_vector_borrowing.rs) | `collections_vector_borrowing` | Vector ownership and borrowing |
| [05_vector_iteration.rs](05_vector_iteration.rs) | `collections_vector_iteration` | Iterating over vectors |
| [06_enum_elements.rs](06_enum_elements.rs) | `collections_enum_elements` | Different enum variants in one vector |
| [07_hashmaps.rs](07_hashmaps.rs) | `collections_hashmaps` | Creating and reading hash maps |
| [08_hashmap_ownership.rs](08_hashmap_ownership.rs) | `collections_hashmap_ownership` | HashMap ownership |
| [09_hashmap_updates.rs](09_hashmap_updates.rs) | `collections_hashmap_updates` | Updating hash map entries |
| [10_word_count.rs](10_word_count.rs) | `collections_word_count` | Counting words |

## 跟课提示

- 先做 Vec，再做 HashMap；每个文件可以单独编译运行。
- 下标越界和借用冲突可按提示临时加入观察；初始框架本身可运行。
- `06_enum_elements.rs` 已提供 Enum 定义和输入数据，自己写 Vec 与 match 部分。
- 词频统计的输入文本已写在 `10_word_count.rs`，不依赖文件路径；HashMap 的遍历输出顺序不固定。
- Lecture 6 仍可通过 `errors_*` 运行目标继续练习。
