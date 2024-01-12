use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl custom_ecs::component::Component for #name {
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
