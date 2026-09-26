# Lecture 4：字符串与数组切片

本组对应 [讲义第 46–56 页](https://iqua.ece.toronto.edu/baochun/ece1724/slides/lecture4.html#/45)，并提前补充使用字符串切片必须知道的 UTF-8 边界。目标是能把一段已有数据借给函数，或从函数返回它的一部分，而不复制整份数据。

| 示例文件 | 运行目标 |
| --- | --- |
| [08_first_word_index.rs](../src/lectures/lec4/08_first_word_index.rs) | `lec4_first_word_index` |
| [09_string_slices.rs](../src/lectures/lec4/09_string_slices.rs) | `lec4_string_slices` |
| [10_str_parameters.rs](../src/lectures/lec4/10_str_parameters.rs) | `lec4_str_parameters` |
| [11_array_slices.rs](../src/lectures/lec4/11_array_slices.rs) | `lec4_array_slices` |

核心示例完整，代码注释为简短英文；仅在 10 留一道 after_first_space 练习。下面的错误片段用于分析，不能与正常示例一起直接运行。

## 01：先确定 first_word 的具体需求

这一版函数返回第一个 ASCII 空格之前的内容；没有空格时返回整个输入。它不是完整的自然语言分词器，也不会自动跳过开头的空白。

| 输入 | 期望结果 |
| --- | --- |
| `"hello world"` | `"hello"` |
| `"Rust"` | `"Rust"` |
| `""` | `""` |
| `" hello"` | `""` |
| `"hello  world"` | `"hello"` |
| `"hello\tworld"` | 原样返回，tab 不是 ASCII 空格 |

需求不同，分隔规则也要改。处理通用空白时可以考虑 split_whitespace，但它会跳过开头空白，语义与这里不同；迭代器等工具以后再展开。

## 02：返回单词结尾的索引

```rust
fn first_word_end(text: &String) -> usize {
    let bytes = text.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return index;
        }
    }

    text.len()
}
```

`text.as_bytes()` 返回借用的字节切片 `&[u8]`，不创建或复制一个新的字节数组。迭代它时，每个元素是字节的引用；enumerate 再加上从零开始的位置，所以循环收到 `(usize, &u8)`。

`(index, &byte)` 是一个解构模式：index 得到位置，&byte 匹配一个引用并得到 u8 值。因为 u8 是 Copy，这里可以复制这个字节。它与表达式中的 `&text` 不同，后者是在创建借用。

也可以把循环写成下面的等价形式，更容易看清解引用：

```rust
for (index, byte_ref) in bytes.iter().enumerate() {
    if *byte_ref == b' ' {
        return index;
    }
}
```

`b' '` 是 u8 字节字面量，`' '` 是 char，`" "` 是字符串字面量。这里遍历字节，因此比较 u8 与 u8。

找不到空格时，用 text.len() 作为结尾。长度和 index 都按字节计算。

## 03：裸索引不会自动和原数据保持同步

```rust
let mut text = String::from("hello world");
let end = first_word_end(&text);
text.clear();
println!("{end}, {}", text.len());
```

输出 5、0，代码可以编译。end 是独立的 usize，不再借用 text；编译器不知道这个数字代表此前那个字符串里的位置。

clear 清空字符串，使长度变为零，但不改变容量；它不是销毁 String 对象。[String::clear](https://doc.rust-lang.org/std/string/struct.String.html#method.clear)

如果随后使用 `&text[..end]`，范围就越界了，运行时会 panic。另一个更隐蔽的问题是向 text 写入 `"hi all"`：旧的 5 又落在合法范围内，但正确的第一个空格位置已经是 2。使用旧范围可能产生错误结果，却不一定报错。

因此，整数 5 本身不是“无效整数”；它是失去了与当前文本相匹配的业务含义。

## 04：字符串切片 &str

```rust
let text = String::from("hello world");
let first: &str = &text[0..5];
let second: &str = &text[6..11];
println!("{first} | {second}");
```

输出 hello | world。范围包括左端点，不包括右端点。原始数据关系如下：

```text
byte index:  0 1 2 3 4 5 6 7 8 9 10
text:        h e l l o _ w o r l d
first:       [0--------5)
second:                  [6--------11)
```

下划线只在示意图中代表空格。first 和 second 借用原文字的连续部分，没有生成独立文字副本。

概念上，&str 包含起始位置和字节长度。second 从原文字的第 6 个字节位置开始，长度为 5；它没有自己的 String 容量，也不负责释放原缓冲区。

| 写法 | 范围 |
| --- | --- |
| `&text[..5]` | 从开头到 5，不包括 5 |
| `&text[6..]` | 从 6 到末尾 |
| `&text[..]` | 整段文字 |
| `&text[5..5]` | 合法边界上的空切片 |

`let other = first;` 会复制 &str 引用值，两份引用仍借用同一段文字。不会把文字自动 clone 一份。[str 的表示与切片](https://doc.rust-lang.org/std/primitive.str.html)

## 05：把返回值改为切片

```rust
fn first_word(text: &str) -> &str {
    for (index, &byte) in text.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &text[..index];
        }
    }

    text
}
```

找到空格时返回此前的部分；找不到时直接返回整个输入引用。函数没有把字符串移进来，也没有克隆文字。

输出借用输入的数据。这种单个输入引用的签名允许编译器根据生命周期省略规则关联输入和输出；本轮不需要手写生命周期参数。[生命周期省略规则](https://doc.rust-lang.org/reference/lifetime-elision.html#lifetime-elision-in-functions)

这也和上一组“返回局部 String 的引用会出错”区分开了：这里的数据来自调用者，函数没有创建一个马上会销毁的局部 String 再返回它的引用。

## 06：返回切片后，借用检查能够保护访问关系

```rust
let mut text = String::from("hello world");
let word = first_word(&text);
text.clear();
println!("{word}");
```

这段不能编译，通常报 E0502。word 后面还要使用，它对 text 的共享借用必须有效；clear 又需要可变借用，两者冲突。

不是“创建过切片以后 String 永远不能改”。调整最后一次使用的位置就可以：

```rust
let mut text = String::from("hello world");
let word = first_word(&text);
println!("{word}");
text.clear();
println!("{text:?}");
```

输出 hello，再输出空字符串。word 在 clear 前已经结束使用，借用区间不再冲突。若最后又加一行打印 word，就重新产生冲突。

编译器也不会把切片当成随原文任意变化的“实时窗口”；它是受借用规则约束的数据视图。[Rust Book：切片与原数据的关系](https://doc.rust-lang.org/book/ch04-03-slices.html)

## 07：为什么参数 &str 比 &String 更通用

函数如果只需要读取文字，不需要 String 特有的管理能力，&str 可以接收更多来源：

```rust
let owned = String::from("hello world");
let literal = "small example";

println!("{}", first_word(&owned));
println!("{}", first_word(owned.as_str()));
println!("{}", first_word(&owned[6..]));
println!("{}", first_word(literal));
```

依次输出 hello、hello、world、small。函数需要 &str 时，&String 可以经解引用强制转换得到借用的字符串视图；owned.as_str() 是显式写法，均不复制文字。字面量本来就是 &str，可以直接传入。

如果参数写成 &String，字面量和已有的 &str 就不能原样传入这个参数。不要为了一个只读函数先制造不必要的 String。

| 类型 | 本组理解 |
| --- | --- |
| `String` | 拥有可增长的 UTF-8 文字 |
| `&String` | 借用整个 String 对象 |
| `&str` | 借用一段有效的 UTF-8 文字，可来自 String、字面量或其他切片 |

不能把所有 &str 都当成字符串字面量，也不能认为它们都能活到程序结束。对局部 String 取出的切片仍受那个 String 的有效期约束。

## 08：确实需要保留独立结果时，转成 String

```rust
let mut text = String::from("hello world");
let saved: String = first_word(&text).to_owned();
text.clear();
println!("{saved}");
```

输出 hello。to_owned 在这里把借来的文字复制成拥有值 String；saved 不再借用 text，因此原字符串可以修改或销毁。也可以根据场景使用 String::from 或 to_string，后面会继续讲字符串操作。

所以“返回 &str”和“返回 String”不是只有写法不同：前者让结果依赖原数据，后者让结果拥有自己的文字。需要独立保留时复制是合理的；只临时读取时则不必复制。

## 09：字符串范围是字节范围，端点必须是 UTF-8 边界

```rust
let text = "中A";
println!("{}", text.len());
println!("{}", &text[..3]);
println!("{}", &text[3..]);
// println!("{}", &text[..1]); // Panics: not a UTF-8 boundary.
```

依次输出 4、中、A。中占字节位置 0、1、2，A 位于字节位置 3，合法切分端点是 0、3、4。

要成功创建 `&text[start..end]`，需要 `start <= end <= text.len()`，而且两个端点都必须是 UTF-8 字符边界。范围在长度以内，不代表一定能切。

即使空范围也要在合法边界上；例如对于这里的 text，`&text[1..1]` 仍然无效。一个 UTF-8 Unicode 标量值可以占 1–4 字节，不能根据“一个字”假定它只占一个字节。

`text.get(..1)` 会返回 None，而不是因无效范围而 panic；有效范围会返回 Some 切片。Option 的系统用法后面再讲，本组只用它观察是否能切。

这里 first_word 搜索 ASCII 空格字节。该字节不会出现在 UTF-8 多字节字符内部，因此 `"你好 world"` 也能安全得到 `"你好"`。这不代表它识别中文全角空格或所有 Unicode 空白，分隔规则仍是单个 ASCII 空格。

## 10：数组也可以借用一段

```rust
let values = [10, 20, 30, 40, 50];
let middle: &[i32] = &values[1..4];
println!("{middle:?}");
println!("{} {}", middle.len(), middle[0]);
```

输出 [20, 30, 40]，然后 3 和 20。

切片范围在这里按元素计算，len 也是元素个数。middle[0] 是切片里的第一个元素，对应原数组 values[1]。不能继续沿用原数组索引来访问切片。

| 类型 | 长度信息 |
| --- | --- |
| `[i32; 5]` | 长度 5 是数组类型的一部分 |
| `&[i32]` | 借用连续整数，具体长度由切片值携带 |

这样函数不必只接受一种固定长度的数组：

```rust
fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for &value in values {
        total += value;
    }
    total
}
```

可以传 `&values` 或 `&values[1..4]`。空切片会使循环执行零次，返回零。这里使用小整数示例，不讨论累计值溢出。

可变切片可以修改原数组的局部区域：

```rust
let mut values = [10, 20, 30, 40, 50];
let middle = &mut values[1..4];
middle[0] = 99;
println!("{values:?}");
```

输出 [10, 99, 30, 40, 50]。这里不是修改一份拷贝；middle 的最后一次使用之后，才直接打印整个数组。不要和上一讲的 `let copied = values;` 混淆：i32 数组按值赋值会 Copy，`&values[range]` 则是借用。[数组切片](https://doc.rust-lang.org/std/primitive.slice.html)

## 11：分清三类问题

| 情况 | 检查方式 |
| --- | --- |
| 原文改变后，旧 usize 不再代表正确位置 | 普通整数不会自动追踪原文，可能留下逻辑错误 |
| 切片后面还要使用，中间修改其原 String | 借用冲突，编译失败 |
| 直接范围索引越界或切进 UTF-8 字符内部 | 通常能编译，但执行该索引会 panic |

借用检查保证合法访问关系，不会替你猜出业务正确的分隔符，也不会证明任意运行时索引都合法。

下一组继续 Lecture 4 第 57–67 页：String 的创建、追加、加号拼接、format!，以及 bytes/chars 和 Unicode 的进一步区别。
