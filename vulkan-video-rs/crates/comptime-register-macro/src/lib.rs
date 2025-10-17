use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};
use syn::{parse::Parse, parse_macro_input, ItemStruct, LitStr};
use std::sync::{LazyLock, Arc, Mutex};

struct ShaderInfo
{
    name: String,
}

static REGISTERED_SHADERS: LazyLock<Arc<Mutex<Vec<ShaderInfo>>>> = LazyLock::new(||{ Arc::new(Mutex::new(Vec::new())) });

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
pub fn register_shader(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as PathArg);

    let struct_name = input_struct.ident.to_string();
    let ident_token = Ident::new(&struct_name, Span::call_site());
    let path = args.path.value();

    let mut shaders = REGISTERED_SHADERS.lock().unwrap();

    let id = shaders.len();
    shaders.push(ShaderInfo {name: struct_name.clone() });

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
pub fn shaders_generate_registry(_input: TokenStream) -> TokenStream {
    let shape_names = REGISTERED_SHADERS.lock().unwrap();

    let generated_code = shape_names.iter().map(|shape_info| {
        let ident_token = Ident::new(&shape_info.name, Span::call_site());
        quote! {
            let instance = #ident_token {};

            let path = #ident_token::PATH;
            let id   = #ident_token::ID;

            paths.push(path);
            ids.push(id);
        }
    });

    let output = quote! {
        pub fn process_all_shaders() -> (Vec<&'static str>, Vec<usize>){
            let mut paths = Vec::new();
            let mut ids   = Vec::new();
            // loop over generated code and running a substitution
            #(#generated_code)* (paths, ids)
        }
    };

    output.into()
}
