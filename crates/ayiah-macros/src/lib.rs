use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Item, ItemMod, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn auto_schema_as(args: TokenStream, input: TokenStream) -> TokenStream {
    let schema_name = args.to_string().trim_matches('"').to_string();
    let mut input = parse_macro_input!(input as ItemMod);

    let struct_name = "Model";

    if let Some((_, items)) = &mut input.content {
        for item in items.iter_mut() {
            if let Item::Struct(item_struct) = item {
                if item_struct.ident == struct_name {
                    let schema_type_ident =
                        Ident::new(&schema_name, proc_macro2::Span::call_site());
                    let schema_attr: syn::Attribute =
                        parse_quote!(#[schema(as = #schema_type_ident)]);
                    item_struct.attrs.push(schema_attr);
                    break;
                }
            }
        }
    }

    quote!(#input).into()
}

#[proc_macro]
pub fn gen_schema(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let mut output = proc_macro2::TokenStream::new();

    for line in input_str.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if let Some((module_name, schema_name)) = line.split_once("=>") {
            let module_name = module_name.trim();
            let schema_name = schema_name.trim().trim_matches('"').trim_end_matches(',');

            let module_ident = Ident::new(module_name, proc_macro2::Span::call_site());
            let schema_name_lit = proc_macro2::Literal::string(schema_name);

            let stmt = quote! {
                #[ayiah_macros::auto_schema_as(#schema_name_lit)]
                pub mod #module_ident {
                    pub use crate::db::entity::#module_ident::*;
                }
            };

            output.extend(stmt);
        }
    }

    output.into()
}
