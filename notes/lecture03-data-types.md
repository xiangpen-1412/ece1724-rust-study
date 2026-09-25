# Lecture 3：数据类型参考

核心示例和典型易错点已经补全，可直接运行、查看输出和英文注释。故意无法编译的写法保留为注释，并说明原因。

| 文件（位于 src/lectures/lec3/） | 运行目标 | 已完成的内容 |
| --- | --- | --- |
| `05_type_inference.rs` | `lec3_type_inference` | 默认类型、后续使用约束类型、明确类型与转换、mut 与 shadowing |
| `06_integer_types.rs` | `lec3_integer_types` | 范围边界、四种溢出方法、可选的运行时溢出实验 |
| `07_numeric_operations.rs` | `lec3_numeric_operations` | 正负整数除法与余数、浮点误差、混合类型运算 |
| `08_type_conversions.rs` | `lec3_type_conversions` | 窄化、转换顺序、有损转换与检查转换 |
| `09_booleans.rs` | `lec3_booleans` | 条件类型、短路求值与非短路求值 |
| `10_characters.rs` | `lec3_characters` | char、字节、UTF-8 长度与组合字符 |

在 RustRover 中打开对应文件，点击 main 旁的运行按钮。也可以使用 `cargo run --bin lec3_type_inference`，替换最后的运行目标即可。

## 综合例题：类型推断、窄化和运算顺序

```rust
fn main() {
    let n = 300;
    let m: u16 = n;
    let k = n as u8;

    println!("n = {n}, m = {m}, k = {k}"); // 300, 300, 44
    println!("Divide first: {}", (k / 10) as f64); // 4
    println!("Convert first: {}", k as f64 / 10.0); // 4.4
}
```

`m: u16 = n` 约束了 n 的推断类型；`as u8` 才执行转换。300 窄化为 u8 后得到 44。整数除法先执行会丢掉小数部分，之后转成浮点数不会恢复它。

复习时先遮住输出注释，预测类型和结果，再运行确认。无需重新抄写所有示例。仅保留的两道动手题在 `11_tuples.rs` 和 `12_arrays.rs` 的结尾。
