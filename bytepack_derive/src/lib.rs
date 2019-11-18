extern crate proc_macro;

#[proc_macro_derive(BytePack)]
pub fn byte_pack_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = ast.ident;

    let data = match ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("BytePack can only be derived for structs."),
    };

    let fields = match data.fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("BytePack can only be derived on structs with named fields."),
    };

    let fieldnames: Vec<syn::Ident> = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone())
        .collect();

    if fieldnames.is_empty() {
        panic!("BytePack requires at least one field");
    }

    let widths: Vec<syn::Expr> = fields.named.iter().map(|field| match field.ty.clone() {
        syn::Type::Array(x) => {
            x.len
        }
        _ => panic!("BytePack can only be derived on structs with named fields where every field is an array"),
    }).collect();

    let gen = quote::quote! {
        impl bytepack::BytePack for #name {
            const WIDTH: usize = #( #widths )+*;

            fn to_slice(&self, dest: &mut [u8]) {
                let mut dest = dest;
                #( dest[..#widths].copy_from_slice(&self.#fieldnames); dest = &mut dest[#widths..]; )*
            }

            fn from_slice(from: &[u8]) -> Self {
                let mut from = from;

                let mut result = Self {
                    #( #fieldnames: [0u8; #widths], )*
                };

                #( result.#fieldnames.copy_from_slice(&from[..#widths]); from = &from[#widths..]; )*;

                result
            }
        }
    };
    gen.into()
}
