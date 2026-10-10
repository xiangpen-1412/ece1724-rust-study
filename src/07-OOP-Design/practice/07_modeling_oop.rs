/*
07 — Enum 建模与 OOP 特征（Lecture 5 后半；Book 18.1 概念）

课件收尾：先完成 01–06。Quiz 3 公告范围是 Chapters 5 and 6，
这里的 Book 18.1 OOP 概念不属于公告列出的重点；时间紧时放在最后。

题 1｜组合判断：让不合法状态无法被构造

某设计用两个 bool 表示下载状态：finished、failed。
这里 finished 表示“已经终止，无论成功还是失败”，failed 表示“以失败终止”。
业务只允许 Pending（未结束）、Succeeded（成功）、Failed（失败）三种状态；
Succeeded 必须有保存路径 String，Failed 必须有错误原因 String。

先列出两个 bool 的四种组合，说明哪种组合无法表达有效业务状态。
再自己设计一个 enum Download，满足上面的三种状态以及各自必需的数据；
不要使用“两个 bool 加两个 Option<String>”的方式。

构造三种状态，写一个借用 Download 的 describe 函数并穷尽 match：
Pending 显示等待；Succeeded 显示路径；Failed 显示原因。
调用后原值必须仍可用于 Debug 打印，不能 clone。

最后判断：“enum 一共有三个 variant，所以它只能表示三个不同的值。”
这个说法正确吗？用自己设计中的 String payload 解释。
说明这里 enum 的 OR 与 struct 字段的 AND 分别指什么。

题 2｜提高：封装、组合与 receiver 一起设计

自己建立 audio module，在其中定义对外可用的 Volume。
内部保存 level: u8，约定合法范围为 0..=10；外部不能直接改 level。
提供 new()（初始为 0）、level() 和 set_level(requested: u8) -> bool。
set_level 接受合法值，拒绝非法值，并在拒绝后保留原 level。

在 audio module 外自己定义 Player，里面保存一个 Volume；
给 Player 写转发的读取/修改方法，让 main 可以通过 Player 调节音量。
亲自检查：初始 0 → 设置 7 → 尝试设置 11 → 最后读取，记录每步返回值和状态。
再临时尝试从 main 直接改 Volume 的私有字段，解释编译器拒绝的边界。

用三句短话回答，不要求写 trait：
1. 为什么“把数据放在 struct、行为放在 impl”可以符合部分 OOP 定义？
2. 为什么 Player 包含 Volume 并不等于 Player 继承了 Volume？
3. Rust 中 trait/泛型与多态的联系是什么，它们会自动给 struct 继承字段吗？
*/

// TODO：自己写 Download、describe、audio module、Player 与方法。

fn main() {
    // 题 1 状态表与判断：
    // TODO：写代码。

    // 题 2 预测与检查：
    // TODO：写代码和概念回答。
}
