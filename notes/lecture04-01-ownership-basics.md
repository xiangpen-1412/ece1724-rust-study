# Lecture 4：Ownership 与 String

这份笔记覆盖 [Lecture 4 第 4–24 页](https://iqua.ece.toronto.edu/baochun/ece1724/slides/lecture4.html#/3) 的实质知识，并补充容易混淆的情况。目标是读到一段代码时，能判断：数据目前归谁负责、赋值有没有复制、旧变量还能不能用，以及何时清理资源。

本轮对应三个可独立运行的参考文件：

| 文件 | 运行目标 | 内容 |
| --- | --- | --- |
| [01_string_and_scope.rs](../src/lectures/lec4/01_string_and_scope.rs) | `lec4_string_and_scope` | String、内存模型、作用域 |
| [02_move_and_clone.rs](../src/lectures/lec4/02_move_and_clone.rs) | `lec4_move_and_clone` | move、clone、重新初始化 |
| [03_copy_and_compound_values.rs](../src/lectures/lec4/03_copy_and_compound_values.rs) | `lec4_copy_and_compound_values` | Copy、元组、数组与部分移动 |

下面的短代码片段可以分别放进 `main` 运行；故意出错的语句保持注释状态。详细解释放在这里，源文件只保留简短英文注释。

**先把问题摆清楚：程序创建了数据，谁负责把它用完并清理？**

整数赋值很容易理解：得到一个新的整数值即可。动态字符串还管理着一块保存文字的内存。如果只是把指向这块内存的信息复制给两个变量，随后两个变量都试图释放同一块内存，就会出现重复释放。

所有权规则让编译器追踪这种责任。它关注的不只是“这个变量现在等于什么”，还包括“这个值现在由哪个位置持有”。这也是为什么两段看起来相同的赋值，可能产生不同结果。

**栈和堆：理解数据怎么组织，不必背地址方向。**

函数调用形成栈帧；调用返回时，相应栈帧退出。这个模型体现后进先出的顺序。堆则用于动态分配，缓冲区的存在时间可以不直接跟随创建它的那个函数调用。

我们经常把简单局部变量画在栈上，把可增长字符串的字节缓冲区画在堆上。实际编译后，一些值可能进入寄存器，或被优化掉；拥有字符串的对象也可以嵌入另一个堆上对象。不能用示意图推断每个变量的实际地址。

课件里的“栈向低地址增长，堆向高地址增长”是某些内存布局的教学图示，**不是 Rust 语言对所有目标平台、分配器或运行情况的保证**。本课需要掌握的是生命周期和资源管理，而不是靠地址高低判断所有权。

**`String` 应当分成对象本身和文字缓冲区来看。**

```rust
let text = String::from("study");
println!("len={}, capacity={}", text.len(), text.capacity());
```

概念上，`String` 保存指向字节缓冲区的信息、当前长度 `len` 和容量 `capacity`；真正的 UTF-8 文字字节由它管理的堆缓冲区保存。

```text
String 对象
  指向缓冲区的信息 ──────────> UTF-8 字节
  len                          s t u d y
  capacity
```

这个图表示关系，不承诺实际字段排列、地址或优化后的存放位置。

`len()` 是当前文字占用的字节数，`capacity()` 是缓冲区容量，满足 `capacity >= len`。容量足够时，追加文字可以使用已有空间；不够时可能重新分配。**不要猜某次追加后容量一定是多少，或者一定按某个倍数增长。** 例如 `String::from("中")` 的长度是 3 字节，不是 1。[String 的表示与容量说明](https://doc.rust-lang.org/std/string/struct.String.html#representation)

**`&str` 字面量和拥有文字的 `String`，职责不同。**

```rust
let literal: &str = "lesson";
let mut owned = String::from(literal);
owned.push_str(" four");
println!("{literal} | {owned}");
```

输出为 `lesson | lesson four`。

`literal` 是对文字的引用。这里的字面量文字在程序整个运行期间有效，不由这个局部变量负责释放。一般的 `&str` 也可以引用其他来源的字符串，不能把所有 `&str` 都理解成字面量。

`owned` 是一个拥有自身缓冲区的 `String`。`String::from(literal)` 根据文字内容创建它；追加时修改的是这个 `String`，不是原字面量。

还要区分“修改绑定”和“修改它引用的文字”：

```rust
let mut label: &str = "first";
println!("{label}");
label = "second";
println!("{label}");
// label.push_str("!"); // Error: no push_str method on &str.
```

这里的 `mut` 允许 `label` 改为引用另一段文字，不会把字面量变成可以追加的字符串。[String 的创建与追加](https://doc.rust-lang.org/std/string/struct.String.html#examples)

**所有权的三个基本规则，要结合“目前的拥有者”来读。**

1. 值有拥有者，负责持有它并承担相应清理责任。
2. 在本轮普通拥有值的模型里，同一个值同时只有一个拥有者。
3. 当仍持有该值的拥有者结束作用域，值会被销毁；`String` 的销毁会释放其缓冲区。

“值有拥有者”不等于“值必须永远由创建它的变量拥有”。所有权可以转移。这三条需要一起使用，不能只看见一个右大括号就断言其中创建的所有堆数据都会消失。[Rust Book：所有权规则](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules)

```rust
let outer = {
    let inner = String::from("survives the block");
    inner
};
println!("{outer}");
```

`inner` 把它持有的 `String` 作为块的结果交出去，随后由 `outer` 持有。内层块结束时，`inner` 的名字确实不能再用，但字符串仍然存在，因为资源清理责任已经转移。

对比下面的独立代码：

```rust
{
    let temporary = String::from("only inside");
    println!("{temporary}");
}
// println!("{temporary}"); // Error: outside its scope.
```

这里没有把值移交给外部，块结束时它被销毁。这个错误是“名字不在作用域中”；后面的 move 错误则是“名字还在作用域中，但原值已移走”。两者要分清。

**`let second = first`：对 `String` 来说，是 move。**

```rust
let first = String::from("draft");
let second = first;
println!("{second}");
// println!("{first}"); // Error: moved value.
```

第二行之后，原来的 `String` 由 `second` 持有。`first` 不能再读取或格式化，因为它已经不持有那个值。编译器在编译阶段检查这个规则。

这次赋值没有要求建立另一份独立的文字缓冲区。可以把 move 理解成把对象描述信息和持有权移交给新位置，同时让旧位置失去使用资格。不要把它理解成原字符串被清空：`first` 不是变成了空字符串，而是暂时没有可读取的有效值。

也不要据此断言处理器一定执行了某条“复制描述信息”的指令。move 描述的是语言语义，具体机器操作可以被优化。

在这个例子结束时，由 `second` 清理字符串；`first` 不会再清理已移交的资源，因此不会因为这次赋值而重复释放。[Rust Book：move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)

**`println!` 不是一个把字符串移走的反例。**

```rust
let message = String::from("ready");
println!("{message}");
println!("{message}");
let destination = message;
println!("{destination}");
```

前三行合法。这里的格式化读取 `message`，临时借用它，而不是取得它的所有权；借用的完整规则后面再讲。真正的所有权转移发生在 `let destination = message`。

所以，“变量出现在另一个表达式中”不等于“一定被 move”。需要看那个操作如何使用值。也不要进一步推广成“传给任意函数都不会 move”；函数参数的情况正是下一部分内容。[格式化接口](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits)

**`clone()`：确实需要两个独立字符串时，明确复制内容。**

```rust
let original = String::from("draft");
let mut revision = original.clone();
revision.push_str(" v2");
println!("{original} | {revision}");
```

输出 `draft | draft v2`。两个 `String` 分别拥有自己的文字存储，因此修改 `revision` 不会改变 `original`；结束时也分别清理各自的资源。

这里说的是 `String::clone` 的具体行为。其他类型的 `clone()` 如何复制，由它们的实现决定，不能把所有 `clone()` 都一概解释成递归复制全部相关数据。

对非空 `String`，复制内容通常涉及新的缓冲区分配，时间和空间成本也随内容增长。解决 moved-value 错误之前，先确定业务是否真的需要保留两份内容；不要遇到错误就机械加 `.clone()`。[String 的 Clone 实现](https://doc.rust-lang.org/std/string/struct.String.html#impl-Clone-for-String)

**`mut` 和 move、Copy 是两个维度。**

`mut` 决定这个绑定能否修改；move 或 Copy 的行为取决于类型。下面不需要把 `source` 声明成可变：

```rust
let source = String::from("note");
let mut destination = source;
destination.push_str("s");
println!("{destination}");
```

把值移交出去不要求旧绑定有 `mut`。移交以后，新绑定可以自行声明为可变；原来的不可变属性不会锁住这个值的所有未来拥有者。

反过来，`let mut source = String::from("note")` 也不会让 `String` 变成 `Copy`。`mut` 不决定是否自动复制。

**已经 move 的可变绑定，可以重新初始化。**

```rust
let mut current = String::from("old");
let saved = current;
// println!("{current}"); // Error: moved value.
current = String::from("new");
println!("{saved} | {current}");
```

输出 `old | new`。`saved` 继续拥有旧字符串；重新赋值给 `current` 创建的是另一个有效值。它没有从 `saved` 抢回旧字符串，也没有让旧值突然回到原变量中。

这也说明“move 后不能再用”更准确的意思是：**不能读取那个已经移走的值；在合法地重新初始化后，可以使用新值。**

对照课件第 21 页的另一种赋值：

```rust
let mut status = String::from("pending");
println!("{status}");
status = String::from("done");
println!("{status}");
```

这里旧值没有事先 move 给别人。赋入新 `String` 时，原来的 `pending` 值被销毁，新值由 `status` 接管；普通赋值覆盖和 shadowing 是不同操作。[Rust Reference：赋值表达式](https://doc.rust-lang.org/reference/expressions/operator-expr.html#assignment-expressions)

**整数的赋值为什么不会让旧变量失效？因为整数实现了 `Copy`。**

```rust
let mut count = 12;
let snapshot = count;
count += 1;
println!("{count} | {snapshot}");
```

输出 `13 | 12`。赋值时获得两个独立的整数值，后续修改其中一个不会改变另一个。

`Copy` 是一种类型能力：当这个类型按值赋值时，可以隐式复制而保持原值可用。整数、浮点数、`bool`、`char` 都有这种能力。这里的 `mut` 只允许后来修改 `count`；删掉 `mut`，第二行复制仍然合法，第三行修改会失败。

课件第 24 页把 Copy 与“栈上简单数据”联系起来，适合理解整数例子，但**判断标准是类型有没有实现 `Copy`，不是变量在哪里存放**。

`String` 自身的描述信息同样是固定大小，却不实现 `Copy`，因为它还负责管理资源。只复制那些位而让两个 `String` 都负责释放同一个缓冲区，不能满足这种安全复制的要求。自定义类型即使只含整数，也不会仅凭“可以放在栈上”就自动拥有 Copy；以后可以学习何时显式实现它。[Copy 的准确含义](https://doc.rust-lang.org/std/marker/trait.Copy.html)

**复制 `&str`，复制的是引用值，不是文字内容。**

```rust
let first: &str = "shared text";
let second = first;
println!("{first} | {second}");
```

`&str` 实现 `Copy`，所以两者都可以继续使用。这里没有自动制造第二份文字缓冲区；两份引用可以指向同一段文字。这与两个拥有独立缓冲区的 `String` 不是同一个情况，也说明“值复制了”不代表“所有关联数据都深复制了”。[Copy 与共享引用](https://doc.rust-lang.org/std/marker/trait.Copy.html#when-can-my-type-be-copy)

**元组和数组要继续看里面的类型。**

| 类型 | 是否 Copy | 理由 |
| --- | --- | --- |
| `(i32, bool)` | 是 | 每个字段都 Copy |
| `(String, i32)` | 否 | String 不 Copy |
| `[i32; 3]` | 是 | 元素类型 i32 是 Copy |
| `[String; 2]` | 否 | 元素类型 String 不 Copy |
| `[&str; 2]` | 是 | 引用类型 &str 是 Copy |

```rust
let first = (7, true);
let second = first;
println!("{first:?} | {second:?}");

let words = [String::from("red"), String::from("blue")];
let moved_words = words;
println!("{moved_words:?}");
// println!("{words:?}"); // Error: moved array.
```

数组大小固定，不足以推导整个数组是 Copy；里面的 `String` 仍然管理资源。对较大的 Copy 数组，隐式复制也可能涉及较多数据，不能把 Copy 理解成“任何尺寸都没有成本”。[数组的 trait 实现](https://doc.rust-lang.org/std/primitive.array.html#trait-implementations)

**拓展：只移走元组的一个字段，会发生部分移动。**

```rust
let record = (String::from("Alice"), 9);
let name = record.0;
println!("{name}");
println!("{}", record.1);
// println!("{record:?}"); // Error: partially moved value.
```

`record.0` 是 `String`，取出它会移动这个字段；`record.1` 还保留着整数，可以单独使用。但是完整 `record` 已经少了一个可用字段，因此不能再把整个元组拿去打印。

这解释了之前的元组实验：只含整数时，解构后常常仍能用原元组；包含 `String` 时，就必须逐字段追踪实际发生了什么。这是复合值的补充，不代表任意类型都允许任意字段移出；某些带有自定义销毁行为的类型另有限制。[部分移动示例](https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html)

**做题时，把状态追踪到每条语句之后。**

```rust
let a = String::from("A");
let b = a;
let mut c = b.clone();
c.push_str("!");
println!("{b} | {c}");
```

| 执行后 | 可以使用什么 | 当前资源关系 |
| --- | --- | --- |
| 创建 a | a | a 拥有第一份字符串 |
| b = a | b | 第一份字符串移交给 b |
| c = b.clone() | b、c | 两份独立字符串 |
| c.push_str | b、c | 只有 c 变成 A! |
| 打印 | b、c | 借用做格式化，没有转移所有权 |

结果是 `A | A!`。如果把 `clone()` 删掉改成 `let mut c = b`，就变成第二次 move；最后同时打印 b、c 会因为 b 已移动而编译失败。

复习时只需要追踪四件事：这一处的类型是什么；这个操作是读取、move、Copy 还是 clone；哪些名字仍有有效值；当前值由谁负责清理。不要用“有没有 mut”“看起来像字符串”“是不是在栈上”替代这些判断。

下一部分从第 25 页开始，把这些规则放进函数参数和返回值中。引用与借用随后展开；这一轮先掌握赋值、作用域和复合值里的所有权变化。
