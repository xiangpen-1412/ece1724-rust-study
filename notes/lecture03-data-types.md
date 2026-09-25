# Lecture 3：数据类型练习

对应本轮讲解：静态类型与类型推断、整数、浮点数、数值运算、转换、布尔和字符。

## 文件与运行目标

| 文件（位于 src/lectures/lec3/） | 运行目标 | 留给自己写的扩展 |
| --- | --- | --- |
| `type_inference.rs` | `lec3_type_inference` | 后续语句约束、明确类型后的赋值、mut 与 shadowing |
| `integer_types.rs` | `lec3_integer_types` | 范围边界、运行时溢出、不同溢出处理方法 |
| `numeric_operations.rs` | `lec3_numeric_operations` | 负数除法与余数、浮点误差、混合类型 |
| `type_conversions.rs` | `lec3_type_conversions` | 窄化、转换顺序、有损转换 |
| `booleans.rs` | `lec3_booleans` | 条件类型、短路与非短路 |
| `characters.rs` | `lec3_characters` | 字符、字节、UTF-8 长度与组合字符 |

基础示例能直接运行。所有代码注释使用英文；TODO 是练习要求，不含扩展题答案。

每次只做一个实验：先预测，再写代码运行，把编译错误也作为观察结果。
需要保持编译错误的例子，可以先记录错误，再注释掉这段实验代码。

## 综合题（自行实现）

1. 声明没有类型标注的 `n = 300`，再声明 `m: u16 = n`。
2. 用 `as u8` 得到 `k`。
3. 对比“先做 k / 10 再转 f64”和“先把 k 转 f64 再除以 10.0”。
4. 改变 m 的类型，重新检查 n 的推断结果以及程序能否编译。

### 我的预测

### 对应代码位置

### 实际结果与错因

### 变式与待解决问题
