/// Show Case

// Derived macros
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct ExampleStruct {
    field1: i32,
    field2: String,
}

// Struct
struct MyStruct<'a, T> {
    reference: &'a T,
    value: T,
}

// Lifetimes
fn lifetime_example<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

// Macros (declarative)
macro_rules! my_macro {
    ($x:expr) => {
        println!("Value: {}", $x);
    };
}

// Procedural Macros (example of a function-like macro - requires external crate like `proc-macro2` and `syn` for real world usage)
// For simplicity, this example just prints the input.
fn function_like_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("Function-like macro input: {}", input.to_string());
    input
}

// Attribute-like Macro (requires external crate like `proc-macro2` and `syn` for real world usage)
// For simplicity, this example just prints the input.
fn attribute_like_macro(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    println!("Attribute-like macro attr: {}", attr.to_string());
    println!("Attribute-like macro item: {}", item.to_string());
    item
}

// Derive Macro (requires external crate like `proc-macro2` and `syn` for real world usage).
// For simplicity, this example just prints the input.
fn derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("Derive macro input: {}", input.to_string());
    input
}

// Using Procedural macros, you would need to define them in a seperate crate of type proc-macro, and then include them using the following syntax.
// #[function_like_macro]
// fn my_function() {
// }
// #[attribute_like_macro(attribute)]
// struct MyStruct {}
// #[derive(MyDeriveMacro)]
// struct MyDeriveStruct {}

fn main() {
    // Struct usage
    let value = 42;
    let my_struct = MyStruct {
        reference: &value,
        value,
    };

    println!("Struct value: {}", my_struct.value);

    // Lifetime usage
    let a = 10;
    let b = 20;
    let max = lifetime_example(&a, &b);
    println!("Max value: {}", max);

    // Macro usage
    my_macro!(100);

    // Derived macro usage.
    let example = ExampleStruct {
        field1: 1,
        field2: "test".to_string(),
    };
    println!("{:?}", example);
}
