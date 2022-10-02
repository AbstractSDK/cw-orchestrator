#![crate_type = "proc-macro"]
mod struct_fn_constructor;

use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, ItemEnum, parse::{Parse, ParseStream}, Error, token::{Struct, Enum}, Token, Item, Type};
use quote::{format_ident, quote, quote_spanned};

// #[proc_macro_derive(Boot)]
// pub fn cosmwasm_contract_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     let expanded = struct_fn_constructor::derive_contract_impl(input).into_token_stream();

//     proc_macro::TokenStream::from(expanded)
// }

struct MyMacroInput {
    exec: Type,
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let exec_type: Type = input.parse()?;
        // let lookahead = input.lookahead1();
        // if lookahead.peek(Token![struct]) {
        //     Ok(MyMacroInput{
        //                     exec: input.parse().map(Item::Struct)?
        //                 })
        // } else if lookahead.peek(Token![enum]) {
        //     Ok(MyMacroInput{
        //         exec: input.parse().map(Item::Enum)?
        //     })
        // } else {
        //     Err(lookahead.error())
        // }
        Ok(MyMacroInput { exec: exec_type })
    }
}

use proc_macro::TokenStream;

#[proc_macro]
pub fn implement_execute(input: TokenStream) -> TokenStream {
    let my_type = parse_macro_input!(input as MyMacroInput);
    let ref name = match &my_type.exec {
        Type::Path(item_enum) => {
            item_enum.path.segments.first().unwrap().ident.clone()
        },
        _ => panic!()
    };
    // panic!("{:?}", my_type.exec);
    let token_stream2 = quote! {
        fn answer() -> String { name }
    };
    TokenStream::from(token_stream2)
}
