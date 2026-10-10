/*
06 — If let 与 pattern 的 ownership（Book 6.3）

题 1｜组合判断：相同语法，内部类型不同

下面三段独立判断：是否能编译？能编译时写完整输出。

A.
    let configured = Some(0u8);
    if let Some(value) = configured {
        println!("value={value}");
    } else {
        println!("missing");
    }
    println!("{configured:?}");

B.
    let configured = Some(String::from("Rust"));
    if let Some(value) = configured {
        println!("value={value}");
    }
    println!("{configured:?}");

C.
    let configured = Some(String::from("Rust"));
    if let Some(_) = configured {
        println!("present");
    }
    println!("{configured:?}");

修复 B，使它能读出内部文字，又能在之后完整使用 configured；不准 clone。
说明把 C 的 _ 换成 _value，会不会只改变“未使用变量”警告。

题 2｜提高：同一需求，match 与 if let 两种实现

写 show_label(label: &Option<String>)：
- Some(text) 时打印 text（空 String 也算存在）；
- None 时打印 "<missing>"。

先用 if let ... else 实现，再独立写一个 match 版本。
对 Some(String::from(""))、Some(String::from("Rust"))、None 分别调用两个版本；
要求输出一致，而且每次调用后原 Option 仍能继续使用。

Book 6.3 补充（新版教材的 let...else）：把同一函数再改写成 let...else，
None 分支打印后提前 return，Some 分支在 let...else 后打印文字。
比较这个写法与 if let 的绑定变量作用域；解释 else 块为何不能只打印后继续执行。
仍然用上面同一组三个输入检查，不另做一套题。

最后写一句话：如果一个 Event enum 新增了变体，原本只处理 Paste 的 if let
会像逐项穷尽的 match 一样强制提醒你补充逻辑吗？哪类需求下这可能遗漏行为？
*/

// TODO：自己写函数。

fn main() {
    // 题 1 预测：
    // TODO：逐段验证。

    // 题 2：
    // TODO：写代码和最后的解释。
}
