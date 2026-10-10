/*
02 — Tuple struct、unit-like struct 与 Debug（Book 5.1–5.2）

题 1｜组合判断：形状相同，是不是同一种类型？

自己定义 Color 和 Point 两个 tuple struct，各有三个 u8 字段；
再定义没有字段的 unit-like struct Marker。先不要 derive Copy 或 Clone。

以下每个片段独立判断，不能因为前一片段编译失败而跳过后面的判断：

    let p: Point = Color(1, 2, 3);                // A

    let c = Color(1, 2, 3);
    let Color(r, g, b) = c;
    println!("{} {}", r + g + b, c.0);           // B

    let m = Marker;
    let another = m;
    let last = m;                               // C

    let empty: () = Marker;                     // D

预测每段的编译结果；能运行的写输出。自己敲代码验证。
然后显式地把一个 Color 转成 Point，不使用类型别名、as 或 unsafe。
说明 B 与 C 的差别是否能仅用“所有字段都是 Copy”解释。

题 2｜提高：Debug 与函数参数的 ownership 一起排错

自己定义 Rectangle { width: u32, height: u32 }，以及
Panel { title: String, rect: Rectangle }。

目标：写一个普通函数 area，让 main 可以连续两次计算同一个 Rectangle 的面积，
随后用 {:?} 和 {:#?} 打印同一个 Panel；打印后仍能访问 panel.title。
请自己选择参数类型，并安排 Rectangle 如何进入 Panel。

验证以下两个故障，每次只制造一个：
1. 只给 Panel derive Debug，不给 Rectangle derive Debug。
2. 把 area 改成按值接收 Rectangle，再保留后续对原 Rectangle 的使用。

分别记录编译器指出的原因，再做最小修复；不使用 clone、Copy 或 Clone derive。
另外判断：修好 Debug 后，把 {:?} 改成 {} 能否直接工作？为什么？
*/

// TODO：自己写类型、derive 与函数。

fn main() {
    // 题 1 预测：
    // TODO：逐段验证。

    // 题 2：
    // TODO：写自己的实现和故障记录。
}
