/*
01 — Struct、字段与 struct update（Book 5.1）

本组只做两题。先写预测，再自己写代码，最后运行验证。
下面的代码片段是待判断的题目，不是参考答案。

题 1｜组合判断：builder、mut 与字段的 ownership

先自己定义 User：username: String、email: String、active: bool、visits: u32。
再自己写普通函数 build_user(username: String, email: String) -> User，
要求使用 field init shorthand，active 初始为 true，visits 初始为 0。

然后判断下面原始片段的每个标记位置：能编译，还是编译错误？说明理由。
为避免连锁影响，检查每个标记时，把其他有问题的标记行注释掉。

    let name = String::from("Ada");
    let user = build_user(name, String::from("ada@example.com"));
    println!("{name}");                 // A
    user.visits += 1;                   // B
    let mail = user.email;              // C
    println!("{}", user.username);      // D（保留 C）
    let whole = user;                   // E（保留 C）

保持 build_user 的参数类型不变，用最小改动让 visits 可以增加，
并让调用方在 build_user 调用之后仍能使用 name。
写清楚改动有没有复制 String 的内容；不要只说“加 mut 就好”。

题 2｜提高：struct update 与 partial move

使用题 1 的 User。每个情境都从一个全新、字段已初始化的 u1 开始。

情境 A：
    let u2 = User { email: String::from("new@example.com"), ..u1 };

情境 B：
    let u2 = User {
        username: String::from("Grace"),
        email: String::from("grace@example.com"),
        ..u1
    };

对 A、B 分别判断以下四个操作能否编译；每项独立检查，互不影响：
    println!("{}", u1.email);
    println!("{}", u1.active);
    println!("{}", u1.username);
    let original = u1;

自己实现并验证两种情境。随后修改情境 A，让 u1 与 u2 都能作为完整的 User
继续使用，同时只复制必要的 String 字段；不要直接 clone 整个 User。
最后用一句话解释：为什么“出现 ..u1 就一定移动整个 u1”不够准确？
*/

// TODO：在这里自己定义 User 和 build_user。

fn main() {
    // 题 1 预测：
    // TODO：写预测与代码。

    // 题 2 预测：
    // TODO：写预测与代码。
}
