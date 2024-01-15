use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name = input.ident;
    let name_string = name.to_string();
    let name_bytes = name_string.as_bytes();

    let mut hash: usize = 5381;

    for c in name_bytes.iter() {
        hash = ((hash << 6).wrapping_add(hash)).wrapping_add(*c as usize);
    }

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #impl_generics component::Component for #name #ty_generics #where_clause {
            const COMPONENT_ID: usize = #hash;
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
