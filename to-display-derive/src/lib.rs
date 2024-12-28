use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(ToDisplay)]
pub fn derive_to_display(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl ::to_display::ToDisplay for #name {
            type Displayer<'a> = &'a #name where Self: 'a;

            fn display_with_context(&self, context: ::to_display::Context) -> Self::Displayer<'_> {
                self
            }
        }
    };

    // Convert back to token stream and return
    TokenStream::from(expanded)
}
