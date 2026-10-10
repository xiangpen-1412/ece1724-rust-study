/*
04 — Enums 与 Option（Book 6.1）

题 1｜组合判断：variant 的数据形状与 String move

自己定义并 derive Debug 的 Message，包含：
- Quit：没有数据；
- Move：命名字段 x: i32、y: i32；
- Write：一个 String；
- Color：三个 u8。
先不要 derive Copy 或 Clone。

以下片段逐个独立判断：
    let a = Message::Move(3, 4);                  // A
    let b = Message::Write("hello");             // B
    let c = [Message::Quit, Message::Color(1, 2, 3)]; // C

    let text = String::from("hello");
    let message = Message::Write(text);
    println!("{text}");                          // D

再为 enum 自己写 inspect 方法，只以 Debug 格式显示当前消息。
选择让同一个 message 可以连续 inspect 两次、随后仍被完整使用的 receiver。
修复 A、B、D，保留各片段原本的数据含义，并解释 C 的元素为什么能放在一起。

题 2｜提高：Option 的类型检查不是空值检查

分析下面的原始片段，找出所有独立的编译问题：

    let missing = None;
    let n: u8 = 5;
    let maybe: Option<u8> = Some(7);
    let total = n + maybe;
    let mixed = [Some(7u8), Some('r')];

给 missing 标出“可能没有 String”的准确类型。
自己写一个含三个字段的 Reading struct：传感器名称、可选的 u8 温度、是否在线。
构造两个在线 Reading，分别表示“读到了 0”和“还没读到值”，用 Debug 打印。
说明为什么不能把 0 当作缺失值，以及 Option<u8> 和 u8 不是同一个类型。

暂时不要用 unwrap/expect。total 的完整处理留在下一组的 match 中完成；
本题仍要解释它为什么报错，不必提前学习新语法来绕过错误。
*/

// TODO：自己写 Message、它的方法和 Reading。

fn main() {
    // 题 1 预测：
    // TODO：写代码。

    // 题 2 预测：
    // TODO：写代码。
}
