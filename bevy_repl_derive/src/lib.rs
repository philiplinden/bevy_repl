use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta};

/// Derive macro for automatically implementing ReplCommand
/// 
/// Usage:
/// ```rust
/// #[derive(ReplCommand)]
/// #[command(name = "hello", about = "Say hello")]
/// struct HelloCommand;
/// ```
#[proc_macro_derive(ReplCommand, attributes(command))]
pub fn derive_repl_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Parse command attributes
    let mut command_name = name.to_string().to_lowercase().replace("command", "");
    let mut about = format!("Execute {} command", command_name);
    let mut aliases: Vec<String> = Vec::new();
    
    // Look for #[command(...)] attributes
    for attr in &input.attrs {
        if attr.path().is_ident("command") {
            if let Meta::List(meta_list) = &attr.meta {
                // Parse the command attributes
                for token in meta_list.tokens.to_string().split(',') {
                    let token = token.trim();
                    if let Some(value) = token.strip_prefix("name = \"").and_then(|s| s.strip_suffix("\"")) {
                        command_name = value.to_string();
                    } else if let Some(value) = token.strip_prefix("about = \"").and_then(|s| s.strip_suffix("\"")) {
                        about = value.to_string();
                    } else if let Some(value) = token.strip_prefix("aliases = [").and_then(|s| s.strip_suffix("]")) {
                        aliases = value.split(',')
                            .map(|s| s.trim().strip_prefix("\"").unwrap_or(s).strip_suffix("\"").unwrap_or(s).to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }
            }
        }
    }
    
    // Generate the implementation
    let expanded = quote! {
        impl Default for #name {
            fn default() -> Self {
                Self
            }
        }
        
        impl Clone for #name {
            fn clone(&self) -> Self {
                Self
            }
        }
        
        impl bevy_repl::ReplCommand for #name {
            fn command(&self) -> clap::Command {
                let mut cmd = clap::Command::new(#command_name)
                    .about(#about);
                    
                #(
                    cmd = cmd.alias(#aliases);
                )*
                
                cmd
            }
            
            fn execute(&self, _commands: &mut bevy::prelude::Commands, _matches: &clap::ArgMatches) -> bevy_repl::ReplResult<String> {
                Ok(format!("{} executed successfully", #command_name))
            }
        }
    };

    TokenStream::from(expanded)
} 
