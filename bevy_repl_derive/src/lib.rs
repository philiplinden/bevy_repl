use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro that automatically implements `ReplCommand` for structs
/// that use clap's `Parser` derive.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use bevy_repl::prelude::*;
/// use clap::Parser;
///
/// #[derive(Parser, ReplCommand, Debug, Clone, Event, Default)]
/// #[command(name = "say", about = "Say something")]
/// struct SayCommand {
///     #[arg(help = "Message to say")]
///     message: String,
/// }
/// ```
#[proc_macro_derive(ReplCommand)]
pub fn derive_repl_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl bevy_repl::command::ReplCommand for #name {
            fn clap_command() -> clap::Command {
                use clap::CommandFactory;
                #name::command()
            }

            fn to_event(matches: &clap::ArgMatches) -> bevy_repl::command::ReplResult<Self> {
                Ok(<#name as clap::FromArgMatches>::from_arg_matches(matches)?)
            }
        }
    };

    TokenStream::from(expanded)
}
