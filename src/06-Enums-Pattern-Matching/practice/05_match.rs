/*
05 — Match：绑定、穷尽性与返回类型（Book 6.2）

题 1｜组合判断：名称是 pattern binding，不是比较

先预测完整输出，并说明外面的 y 有没有变化，再运行：

    let y = 10;
    let x = Some(5);
    let result = match x {
        Some(y) => y + 1,
        None => y,
    };
    println!("{result} {y} {x:?}");

独立判断三个修改（每次恢复原片段，只做一个修改）：
A. 删掉 None 这一整个 arm。
B. 把 Some(y) 对应的表达式改成 { y + 1; }。
C. 把第一条 arm 改成 other => 99，并把 Some(y) => y + 1 放在它后面，
   同时保留 None => y。这属于编译错误、警告还是运行时 panic？输出如何？

解释 named catch-all 与 _ 的区别；说明“match 完整”是否能保证业务逻辑正确。

题 2｜提高：用穷尽 match 写一个小型指令处理器

自己定义 Command：Keep、Add(i32)、Replace(Option<i32>)。
写 apply(current: Option<i32>, command: Command) -> Option<i32>：
- Keep 保留 current；
- Add(delta) 仅在 current 有值时相加；缺失时继续缺失；
- Replace(next) 直接采用 next（可能是 None）。

限制：用 match，不使用 unwrap、expect、panic、if let 或 _ 兜底。
本题假设相加结果不超出 i32 范围，不需要另写溢出处理。
自己检查下面输入；先预测再运行：
(Some(0), Add(3))；(None, Add(3))；(Some(7), Replace(None))；(None, Keep)。
然后临时增加一个没有数据的 Reset variant，先不改 apply，观察编译器反应。
最后规定 Reset 表示 Some(0)，由你补全实现。
*/

// TODO：自己写 Command 与 apply。

fn main() {
    // 题 1 预测：
    // TODO：写代码。

    // 题 2 输入/输出预测：
    // TODO：写代码。
}
