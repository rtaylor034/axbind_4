use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Ident;

#[proc_macro_derive(OptWrite)]
pub fn optwrite_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_optwrite(ast)
}

fn impl_optwrite(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => do_named_struct(name, generics, fields),
            _ => panic!("Currently, only named-field structs are supported."),
        },
        _ => panic!("Currently, only structs are supported."),
    }
}
fn do_named_struct(
    s_ident: &Ident,
    s_generics: &syn::Generics,
    fields: &syn::FieldsNamed,
) -> TokenStream {
    let names: Vec<&Ident> = fields
        .named
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();
    quote! {
        impl #s_generics OptWrite for #s_ident #s_generics {
            fn optwrite(&mut self, other: Self) {
                #(self.#names.optwrite(other.#names));*
            }
            fn overriden_by(self, other: Self) -> Self {
                Self {
                    #(#names: self.#names.overriden_by(other.#names),)*
                }
            }
        }
    }
    .into()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
