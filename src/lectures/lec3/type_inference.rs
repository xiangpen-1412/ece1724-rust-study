// Lecture 3, slides 10-21: static typing and type inference.
// Run target: lec3_type_inference
// Run the basic example, then implement the extension exercises below.

fn main() {
    // Basics: let the compiler infer a type, or specify it explicitly.
    let count = 10;
    let small: u8 = 10;
    let price = 3.5;

    // Use type_name_of_val to observe the type chosen by the compiler.
    println!(
        "count = {count}, type = {}",
        std::any::type_name_of_val(&count)
    );
    println!(
        "small = {small}, type = {}",
        std::any::type_name_of_val(&small)
    );
    println!(
        "price = {price}, type = {}",
        std::any::type_name_of_val(&price)
    );

    // Exercise A: can a later use constrain the type of an earlier binding?
    // TODO: declare n = 300 without a type annotation, then declare m: u16 = n.
    // Print both values and types. Change u16 to u8 and observe what happens.
    // My prediction:
    // My code:
    // Actual result:
    // Explanation:

    // Exercise B: are type inference and type conversion the same operation?
    // TODO: use 10 in both cases: n without an annotation, and n annotated as i32.
    // Try assigning each n to a u8 variable. Record which case compiles and why.
    // My code:

    // Exercise C: how do mut and shadowing affect a binding's type?
    // TODO: create a mutable integer variable, then try assigning a float to it.
    // Next, use let with the same name to create a float binding. Compare the results.
    // My code:
}
