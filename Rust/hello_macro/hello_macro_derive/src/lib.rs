/* For this macro to work, we need to add the following to the `hello_macro/hello_macro_derive/Cargo.toml` file;
 *          [lib]
 *          proc-macro = true
 * 
 *          [dependencies]
 *          syn = "1.0"
 *          quote = "1.0"
 * To start defining the procedural macro, we need the code for `hello_macro_derive()`. Inside it we find the following:
 *      - The `proc_macro_derive(HelloMacro) annotation to be able to call the macro.
 *      - `syn::parse(input).unwrap()` converts the TokenStream to a data structure that we can interpret and perform operations on. The `unwrap()` call 
 *        should be changed to a more speccific message by using `panic!` or `expect`.
 *  
 * We then pass the data structure to our implementation of the macro.
 */
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Counstruct a representation of Rust code as syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

/** In the function, we first get the name of the struct the macro is being implemented in. This is an attribute of the data structure passed into the function, so
 *  the name variable will be "STRUCT_NAME". The `quote!` macro lets us define the Rust code that we return. We can convert it into a TokenStream by calling the 
 *  `into()` method, which consumes the intermediate representation and returns a value of the required TokenStream type. It also provides us some templating mechanics,
 *  like using `#name` to make quote! replace it by the value in the name varible. Then, we implement the trait for the struct defined in `name` and make it print.
 *  By using the built-in `stringify!` macro we can turn an expression into a string literal at compiile time.
 */
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}