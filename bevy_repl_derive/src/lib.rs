use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

/// Derive macro that automatically implements `FromArgMatches` for unit structs
/// that implement `ReplCommand`.
/// 
/// # Example
/// ```rust
/// use bevy_repl_derive::ReplCommand;
/// 
/// #[derive(ReplCommand)]
/// struct MyCommand;
/// ```
#[proc_macro_derive(ReplCommand)]
pub fn derive_repl_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let expanded = quote! {
        impl clap::FromArgMatches for #name {
            fn from_arg_matches(_matches: &clap::ArgMatches) -> Result<Self, clap::error::Error> {
                Ok(#name)
            }
            
            fn update_from_arg_matches(&mut self, _matches: &clap::ArgMatches) -> Result<(), clap::error::Error> {
                Ok(())
            }
        }
    };
    
    TokenStream::from(expanded)
}


