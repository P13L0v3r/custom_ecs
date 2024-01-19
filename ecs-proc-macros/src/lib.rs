use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use rand::{thread_rng, Rng};
use syn::{parse_macro_input, DeriveInput, Expr, Ident, LitStr, PatStruct};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name = input.ident;
    let mut component_string = name.to_string();

    let data_string = match input.data {
        syn::Data::Struct(data) => data.fields.to_token_stream().to_string(),
        syn::Data::Enum(data) => data.variants.to_token_stream().to_string(),
        syn::Data::Union(_) => String::new(),
    };

    component_string.push_str(&data_string);

    let mut rng = thread_rng();
    let rand_char = rng.gen::<char>();
    component_string.push(rand_char);

    let component_bytes = component_string.as_bytes();

    let mut hash: usize = 5381;

    for c in component_bytes.iter() {
        hash = ((hash << 6).wrapping_add(hash)).wrapping_add(*c as usize);
    }

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #impl_generics Component for #name #ty_generics #where_clause {
            fn hash() -> usize where Self : Sized {
                #hash
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn name_to_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    TokenStream::from_str(&input.value()).unwrap_or(TokenStream::new())
}

#[proc_macro]
pub fn evaluate_string_var(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let ident_span = ident.span();
    println!("{:?}", ident_span.source_text());

    TokenStream::new()
}
