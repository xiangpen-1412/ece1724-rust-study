/*
03 — Methods 与 associated functions（Book 5.3）

题 1｜组合判断：receiver 与借用的最后一次使用

自己定义不实现 Copy/Clone 的 Rectangle（width、height 均为 u32）。
实现 area、resize(width, height)、into_dimensions 三个方法：
- area 只读，返回面积；
- resize 修改尺寸；
- into_dimensions 消耗 Rectangle，返回 (u32, u32)。
自己为它们选择正确 receiver。

分析以下片段（遇到错误先记录，不要直接删到能编译为止）：

    let mut r = Rectangle { width: 3, height: 4 };
    let view = &r;
    r.resize(6, 8);                            // A
    println!("{}", view.area());              // B
    let dimensions = r.into_dimensions();
    println!("{}", r.area());                 // C

指出借用冲突所涉及的两处代码，以及另一个不同类别的错误。
要求不 clone、不新增 Rectangle：只调整 view 的最后使用位置，并调整
消耗方法的调用时机，使程序先观察原面积，再观察新面积，最后取得 dimensions。

题 2｜提高：关联函数、自动借用与普通参数

为 Rectangle 增加关联函数 new(width, height) -> Self 和 square(size) -> Self。
在另一个 impl block 中添加 can_hold：只读两个 Rectangle；不允许旋转，
两个维度都严格更大才算能容纳（相等也不行）。

先判断下面哪些调用成立，再自己运行验证；四个调用分别独立检查：
    let outer = Rectangle::new(6, 8);
    let inner = Rectangle::square(3);
    outer.can_hold(inner);
    outer.can_hold(&inner);
    Rectangle::can_hold(&outer, &inner);
    outer.square(4);

调用成立时 outer、inner 是否还能继续使用？说明自动借用适用于哪里。
最后在 main 自己选择检查方式，覆盖这三个判断：
6×8 容纳 3×3；6×8 容纳 6×7；6×8 容纳 7×5。
*/

// TODO：自己写 Rectangle 和两个 impl block。

fn main() {
    // 题 1 预测与修复理由：
    // TODO：写代码。

    // 题 2 预测：
    // TODO：写代码。
}
