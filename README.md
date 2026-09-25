# ECE1724 Rust 学习项目

保存自己写的课堂练习和学习笔记，使用 RustRover，通过 GitHub 在两台电脑之间同步。

仓库：[xiangpen-1412/ece1724-rust-study](https://github.com/xiangpen-1412/ece1724-rust-study)

SSH 地址：`git@github.com:xiangpen-1412/ece1724-rust-study.git`

## 项目结构

```text
src/main.rs       当前练习入口，初始只有空 main 函数
examples/         各个知识点的独立练习，初始为空
notes/            自己的课堂笔记
```

在 RustRover 中打开项目根目录。打开 `src/main.rs`，点击 `fn main()` 旁的绿色运行按钮即可运行。

后面在 `examples/` 新建独立 `.rs` 文件，例如 `lecture03_01_variables.rs`、`lecture03_02_shadowing.rs`。
每个文件写自己的 `fn main()`，可以从对应的运行按钮单独运行，不用把所有知识点堆在一个文件里。

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
# 开始前
git pull --ff-only

# 结束时：先确认将提交的文件
git status
git diff
git add src examples notes
git diff --cached
git commit -m "Lecture 3: practice notes"
git push
```

如果修改了 Cargo.toml、Cargo.lock 或其他配置，也需要把相应文件加入提交。

## Rust 环境

项目使用 Rust 1.98.1、Rust 2024 edition，版本由 `rust-toolchain.toml` 固定。
另一台电脑需要安装 Rust 和对应操作系统的链接工具；Windows 使用原生 MSVC C++ Build Tools。

`Cargo.lock` 跟随项目提交；本机生成的 `target/`、`.idea/` 不上传。

```text
cargo run
cargo run --example lecture03_01_variables
cargo check --all-targets
cargo fmt --check
```

其中 `--example` 命令需要先创建对应的练习文件。
