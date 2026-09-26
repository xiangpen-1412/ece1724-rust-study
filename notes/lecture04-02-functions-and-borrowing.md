# Lecture 4：函数所有权、引用与借用

接着上一组学习 [Lecture 4 第 25–45 页](https://iqua.ece.toronto.edu/baochun/ece1724/slides/lecture4.html#/24)。这一组解决实际写函数时的三个选择：接管一个值，只读取它，或者修改调用者的原值。

| 文件 | 运行目标 |
| --- | --- |
| [04_function_ownership.rs](../src/lectures/lec4/04_function_ownership.rs) | `lec4_function_ownership` |
| [05_shared_references.rs](../src/lectures/lec4/05_shared_references.rs) | `lec4_shared_references` |
| [06_mutable_references.rs](../src/lectures/lec4/06_mutable_references.rs) | `lec4_mutable_references` |
| [07_borrowing_rules.rs](../src/lectures/lec4/07_borrowing_rules.rs) | `lec4_borrowing_rules` |

代码注释保持简短英文。核心示例均完整；本组仅在 06 留一道练习。下面的独立片段按说明放入 main，辅助函数放在 main 外；故意报错的例子不要全部同时启用。

## 01：函数参数也遵守 move / Copy

```rust
fn display_owned(text: String) {
    println!("{text}");
}

fn main() {
    let title = String::from("Rust");
    display_owned(title);
    // println!("{title}"); // E0382: title was moved.
}
```

`display_owned` 声明需要一个 String 值。调用时 title 的值移给参数 text；函数末尾仍由 text 持有的字符串被销毁。即使函数体只打印、不修改，传参也已经转移了所有权。不能根据函数名字或函数体“看起来只是读取”判断参数是否消费原值，要先看签名。

参数名也不必与调用者的变量名相同。`title` 和 `text` 是不同作用域里的绑定，名字不同不会影响所有权规则。

对于 i32，按值传入的是副本：

```rust
fn increase_local(mut number: i32) {
    number += 1;
    println!("{number}");
}

fn main() {
    let number = 10;
    increase_local(number);
    println!("{number}");
}
```

依次输出 11、10。函数参数里的 mut 只允许修改这个参数；不会自动修改调用者的变量。[Rust Book：所有权与函数](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)

## 02：返回值可以把所有权交给调用者

```rust
fn add_mark(mut text: String) -> String {
    text.push('!');
    text
}

fn main() {
    let source = String::from("Rust");
    let result = add_mark(source);
    println!("{result}");
    // println!("{source}"); // E0382: source remains moved.
}
```

所有权按 source → 参数 text → 返回结果 result 的路径传递。输出 Rust!。返回时，值被移交出函数，因此函数结束不会把已经交出去的字符串销毁；返回 String 不意味着必须 clone 它。

如果写 `let source = add_mark(source);`，右边先使用旧绑定，左边再建立新绑定。这是 shadowing，不是旧绑定凭空恢复了原值。

返回 unit `()` 不会自动返还已接管的 String。函数要交回拥有值，就需要明确把它作为返回值的一部分。

## 03：课件的元组陷阱——先移动，再借用

下面的函数不能编译：

```rust
fn with_length(text: String) -> (String, usize) {
    (text, text.len())
}
```

元组元素按从左到右的顺序求值。第一个元素 text 已把 String 移进元组，第二个元素再调用 text.len() 就会借用已移出的值，触发 E0382。

可靠写法：

```rust
fn with_length(text: String) -> (String, usize) {
    let length = text.len();
    (text, length)
}
```

调用者可以写 `let (text, length) = with_length(text);`，用 shadowing 接收返回的 String 和长度。这里 length 是字节数。

一个变式是 `(text.len(), text)`：先读取长度再移动，顺序本身可以成立，但返回类型也要相应改成 `(usize, String)`。不要只换元素而忘了类型。

课件第 28、30 页的标题提到不取得所有权，但这段“传入 String，再用元组返回”的代码实际上仍然经历了所有权转移。下一步的引用才真正避免接管原 String。[Rust Reference：操作数求值顺序](https://doc.rust-lang.org/reference/expressions.html#evaluation-order-of-operands)

## 04：共享引用——使用数据，但不接管它

```rust
fn byte_length(text: &String) -> usize {
    text.len()
}

fn main() {
    let text = String::from("Rust");
    let length = byte_length(&text);
    println!("{text}: {length}");
}
```

三个位置的含义要分开：

| 写法 | 含义 |
| --- | --- |
| `text: String` | 参数接收一个拥有值 |
| `text: &String` | 参数接收指向 String 的共享引用 |
| `&text` | 在表达式中借用现有 text，产生共享引用 |

`&text` 没有复制文字缓冲区，也没有移走 text 持有的 String。函数只借用它，返回后调用者仍然拥有原值；不需要用元组把 String 交回来。

对于这里的普通 String，共享引用允许读取，不允许通过它修改字符串。一般叫共享引用，也常叫不可变引用。以后有内部可变性的类型，会有额外规则，本组不展开。

沿用课件先写 &String，便于看出从 String 到 &String 的变化。只读文字接口通常更适合 &str；下一部分的字符串切片会解释原因。

## 05：可变引用——修改调用者的原值

```rust
fn add_mark(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut text = String::from("Rust");
    add_mark(&mut text);
    println!("{text}");
}
```

输出 Rust!。调用者继续拥有这个 String，函数通过可变引用修改它。

这里三处配合完成操作：`let mut text` 允许这个局部变量中的值被修改；调用处 `&mut text` 创建可变借用；参数类型 `&mut String` 接收可变引用。只改其中一处通常不够。

不要混淆 mut 的位置：

| 参数声明 | 意义 |
| --- | --- |
| `text: String` | 接管 String，参数绑定不可变 |
| `mut text: String` | 接管 String，参数绑定可变 |
| `text: &String` | 共享借用 String |
| `mut text: &String` | 参数可改为引用别处，但仍不能通过它修改 String |
| `text: &mut String` | 可通过引用修改 String；参数本身无需写 mut |

例如 `fn wrong(mut text: &String) { text.push('!'); }` 仍会报错，原因是 &String 没有提供修改 String 的权限。[Rust Book：引用与借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## 06：解引用符号 *

```rust
fn increment(number: &mut i32) {
    *number += 1;
}

fn main() {
    let mut count = 10;
    increment(&mut count);
    println!("{count}");
}
```

输出 11。number 是引用，`*number` 表示引用指向的那个整数位置。在这里修改的是调用者的 count。

String 的例子写 `text.push('!')` 即可，是因为方法调用会进行适当的自动解引用与借用。这不意味着引用与所指值变成了同一个东西，也不意味着任何运算都能省掉 *。

## 07：同一份数据，借用使用区间不能冲突

这一组普通引用的核心约束：可以有多个共享借用，或者一个独占的可变借用；不能让它们发生冲突。这里讨论的是同一份数据。不同变量可以分别被可变借用。

多个共享引用可以一起读取：

```rust
let text = String::from("Rust");
let first = &text;
let second = &text;
println!("{first} | {second}");
```

下面两个可变借用存在重叠，不能编译：

```rust
let mut text = String::from("Rust");
let first = &mut text;
let second = &mut text;
println!("{first} | {second}");
```

最后一行还要用 first，因此建立 second 时，first 的借用仍然需要有效，触发 E0499。并不是必须写上两个线程才会触发借用检查。

共享与可变借用重叠也不行：

```rust
let mut text = String::from("Rust");
let reader = &text;
let writer = &mut text;
writer.push('!');
println!("{reader}");
```

reader 后面还会用到，因此中间不能创建与它冲突的可变借用，触发 E0502。

可变借用的独占性也会限制拥有者直接访问同一数据：

```rust
let mut text = String::from("Rust");
let writer = &mut text;
println!("{text}");
writer.push('!');
```

writer 还会被使用，不能在中途又通过 text 读取同一 String。将打印移到 writer 的最后一次使用之后，就可以通过。

## 08：借用何时结束——看使用区间，不只看大括号

```rust
let mut text = String::from("Rust");
let first = &text;
let second = &text;
println!("{first} | {second}");

let writer = &mut text;
writer.push('!');
println!("{text}");
```

这段合法。first 和 second 在第一次打印后不再使用；writer 在 push 后不再使用。所需借用区间不重叠，所以同一个块里仍可以先读、再写、再读。

这是非词法生命周期（NLL）的一个表现。直线代码里可以先按“最后一次使用”判断；有分支和循环时编译器还会考虑控制流，不是只找文本中最后一次出现的位置。

如果在末尾再加 `println!("{first}");`，first 的使用需求延续到了后面，会与中间的可变借用冲突。

引用的名字还在词法作用域里，不意味着借用一定要持续到那个作用域末尾。用额外大括号也可以明确地分开借用，但很多时候不需要。

## 09：这些限制为什么有用

可变访问需要独占，可以阻止某处修改或重新分配数据时，另一处仍依赖先前访问方式。并发中的数据竞争涉及不同线程对同一内存发生冲突访问、至少一次写入且至少一次访问是非原子的，并且缺少必要的同步关系；安全 Rust 的类型与借用规则等机制用于防止这种情况。原子操作与线程同步后面再学，这里先理解访问冲突。

数据竞争与更广义的竞态问题不同。安全 Rust 不保证所有程序都没有逻辑错误、死锁或时序错误，也不能把“多个引用存在”本身等同于“已经发生数据竞争”。[Rustonomicon：Races](https://doc.rust-lang.org/nomicon/races.html)

## 10：引用不能指向已经销毁的局部数据

错误设计：

```rust
fn make_text() -> &String {
    let local = String::from("Rust");
    &local
}
```

函数创建的 local 在函数结束时会被销毁，却试图交出指向它的引用。这会产生悬垂引用，因此无法编译。原样写通常先报告 E0106；给返回类型随便添一个生命周期名或 'static，不会延长这个局部 String 的生命。

应该返回拥有值：

```rust
fn make_text() -> String {
    let local = String::from("Rust");
    local
}
```

返回时所有权转移给调用者，值可以继续存在。规则不是“函数不能返回引用”，而是“返回的引用在被使用期间必须有效”。例如借用调用者仍然有效的数据，是另一种可成立的设计，之后会展开。

## 写函数前的选择

| 需求 | 本组写法 | 调用后原变量 |
| --- | --- | --- |
| 接管 String | `fn f(text: String)` | 已移出，除非接收返回值重新绑定 |
| 只读 String | `fn f(text: &String)` | 仍拥有原值 |
| 修改原 String | `fn f(text: &mut String)` | 仍拥有修改后的值 |
| 返回新建的 String | `fn f() -> String` | 调用者接收所有权 |

复习时先判断参数类型，再画出值转移或借用的使用区间。不要为了消掉所有错误而一律 clone，也不要用 mut 代替对参数类型的判断。

下一部分从第 46 页开始：用 first_word 问题引出字符串切片 &str，解释为什么返回裸索引容易失效，以及只读字符串参数为什么通常写 &str。
