use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Error, Fields, Ident, LitStr};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into();
    };
}

#[proc_macro_derive(LabeledVariants)]
pub fn labeled_variants_macro_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    match derive_input.data {
        Data::Enum(data_enum) => impl_labeled_variants(derive_input.ident, data_enum),
        _ => derive_error!("LabeledVariants can only be applied to enums!"),
    }
}

fn impl_labeled_variants(name: Ident, data_enum: DataEnum) -> TokenStream {
    let arms = data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let label = LitStr::new(
                &format_label(variant_ident.to_string()),
                variant_ident.span(),
            );
            let params = match variant.fields {
                Fields::Unit => quote! {},
                Fields::Named(_) => quote! { {..} },
                Fields::Unnamed(_) => quote! { (..) },
            };
            quote! { #name::#variant_ident #params => #label }
        })
        .collect::<Vec<_>>();
    let labeled_variants = quote! {
        impl LabeledVariants for #name {
            fn variant_label(&self) -> &'static str {
                match self {
                  #(#arms),*
                }
            }
        }
    };
    labeled_variants.into()
}

fn format_label(ident_name: String) -> String {
    ident_name.to_case(Case::Snake)
}
