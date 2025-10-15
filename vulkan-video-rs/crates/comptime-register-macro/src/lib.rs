use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};
use syn::{parse::Parse, parse_macro_input, ItemStruct, LitStr};
use std::sync::{LazyLock, Arc, Mutex};

struct ShapeInfo
{
    name: String,
    #[allow(dead_code)]
    path: String
}

static REGISTERED_SHAPES: LazyLock<Arc<Mutex<Vec<ShapeInfo>>>> = LazyLock::new(||{ Arc::new(Mutex::new(Vec::new())) });

struct PathArg {
    path: LitStr,
}

impl Parse for PathArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // We expect a single string literal (the path).
        let path: LitStr = input.parse()?;
        Ok(PathArg { path })
    }
}

#[proc_macro_attribute]
pub fn register_shape(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as PathArg);

    let struct_name = input_struct.ident.to_string();
    let ident_token = Ident::new(&struct_name, Span::call_site());
    let path = args.path.value();

    let mut shapes = REGISTERED_SHAPES.lock().unwrap();

    let id = shapes.len();
    shapes.push(ShapeInfo {name: struct_name.clone(), path: path.clone()});

    let output_tokens = quote! {
        #input_struct

        impl #ident_token {
            pub const PATH: &str  = #path;
            pub const ID  : usize = #id; 
        }
    };

    output_tokens.into()
}

#[proc_macro]
pub fn generate_registry(_input: TokenStream) -> TokenStream {
    let shape_names = REGISTERED_SHAPES.lock().unwrap();

    let generated_code = shape_names.iter().map(|shape_info| {
        let ident_token = Ident::new(&shape_info.name, Span::call_site());

        quote! {
            let instance = #ident_token {};
            let name = instance.name();
            println!("Registering type: {}", name);
            registry.push(format!("Registered {}", name));

            #ident_token::print_info();
        }
    });

    let output = quote! {
        pub fn process_all_shapes() -> Vec<String> {
            let mut registry = Vec::new();
            // loop over generated code and running a substitution
            #(#generated_code)* registry
        }
    };

    output.into()
}
