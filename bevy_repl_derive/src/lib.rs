use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, Meta, Field, Fields, Type, 
    Attribute
};

/// Derive macro for automatically implementing ReplCommand with clap-style syntax
/// 
/// Usage:
/// ```rust
/// // Simple command with no arguments
/// #[derive(ReplCommand)]
/// #[command(name = "hello", about = "Say hello")]
/// struct HelloCommand;
/// 
/// // Command with arguments
/// #[derive(ReplCommand)]
/// #[command(name = "spawn", about = "Spawn an entity")]
/// struct SpawnCommand {
///     #[arg(help = "Entity name")]
///     name: String,
///     #[arg(short, long, default_value = "100")]
///     health: i32,
///     #[arg(long)]
///     position: Option<Vec3>,
/// }
/// ```
#[proc_macro_derive(ReplCommand, attributes(command, arg))]
pub fn derive_repl_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Parse command attributes
    let (command_name, about, aliases) = parse_command_attrs(&input.attrs, name);
    
    // Parse struct fields to generate arguments
    let (arg_definitions, field_parsers, field_defaults) = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => parse_struct_fields(&fields.named),
            Fields::Unnamed(_) => (quote!{}, quote!{}, quote!{}),
            Fields::Unit => (quote!{}, quote!{}, quote!{}),
        },
        _ => (quote!{}, quote!{}, quote!{}),
    };
    
    // Generate the implementation
    let expanded = if has_fields(&input.data) {
        // For structs with fields, generate argument parsing
        quote! {
            impl Default for #name {
                fn default() -> Self {
                    Self {
                        #field_defaults
                    }
                }
            }
            
            impl Clone for #name {
                fn clone(&self) -> Self {
                    Self {
                        #field_parsers
                    }
                }
            }
            
            impl bevy_repl::ReplCommand for #name {
                fn command(&self) -> clap::Command {
                    let mut cmd = clap::Command::new(#command_name)
                        .about(#about);
                        
                    #(
                        cmd = cmd.alias(#aliases);
                    )*
                    
                    #arg_definitions
                    
                    cmd
                }
                
                fn execute(&self, commands: &mut bevy::prelude::Commands, matches: &clap::ArgMatches) -> bevy_repl::ReplResult<String> {
                    // Create instance with parsed arguments
                    let instance = Self::from_matches(matches)?;
                    instance.run(commands)
                }
            }
            
            impl #name {
                fn from_matches(matches: &clap::ArgMatches) -> bevy_repl::ReplResult<Self> {
                    Ok(Self {
                        #field_parsers
                    })
                }
                
                fn run(&self, _commands: &mut bevy::prelude::Commands) -> bevy_repl::ReplResult<String> {
                    Ok(format!("{} executed successfully", #command_name))
                }
            }
        }
    } else {
        // For unit structs, keep the simple implementation
        quote! {
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
        }
    };

    TokenStream::from(expanded)
}

// Helper function to parse #[command(...)] attributes
fn parse_command_attrs(attrs: &[Attribute], name: &syn::Ident) -> (String, String, Vec<String>) {
    let mut command_name = name.to_string().to_lowercase().replace("command", "");
    let mut about = format!("Execute {} command", command_name);
    let mut aliases: Vec<String> = Vec::new();
    
    for attr in attrs {
        if attr.path().is_ident("command") {
            if let Meta::List(meta_list) = &attr.meta {
                // Simple string parsing for the attributes
                let tokens_str = meta_list.tokens.to_string();
                let parts: Vec<&str> = tokens_str.split(',').collect();
                
                for part in parts {
                    let part = part.trim();
                    if let Some(value) = extract_string_value(part, "name") {
                        command_name = value;
                    } else if let Some(value) = extract_string_value(part, "about") {
                        about = value;
                    } else if let Some(values) = extract_array_value(part, "aliases") {
                        aliases = values;
                    }
                }
            }
        }
    }
    
    (command_name, about, aliases)
}

// Helper function to parse struct fields and generate clap arguments
fn parse_struct_fields(fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut arg_definitions = Vec::new();
    let mut field_parsers = Vec::new();
    let mut field_defaults = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_type = &field.ty;
        
        // Parse #[arg(...)] attributes
        let (short, long, default_value, help, required) = parse_arg_attrs(&field.attrs, &field_name_str);
        
        // Determine if the field is optional
        let is_optional = is_option_type(field_type);
        let is_required = required.unwrap_or(!is_optional && default_value.is_none());
        
        // Build the argument definition step by step
        let mut arg_tokens = vec![
            quote! { clap::Arg::new(#field_name_str) },
            quote! { .help(#help) },
            quote! { .required(#is_required) },
        ];
        
        if let Some(short_char) = short {
            arg_tokens.push(quote! { .short(#short_char) });
        }
        
        if let Some(long_name) = &long {
            arg_tokens.push(quote! { .long(#long_name) });
        }
        
        if let Some(default) = &default_value {
            arg_tokens.push(quote! { .default_value(#default) });
        }
        
        let arg_def = quote! {
            cmd = cmd.arg(
                #(#arg_tokens)*
            );
        };
        
        arg_definitions.push(arg_def);
        
        // Generate field parser for from_matches
        let parser = if is_optional {
            quote! {
                #field_name: matches.get_one::<String>(#field_name_str)
                    .and_then(|s| s.parse().ok())
            }
        } else if let Some(default) = &default_value {
            quote! {
                #field_name: matches.get_one::<String>(#field_name_str)
                    .unwrap_or(&#default.to_string())
                    .parse()
                    .map_err(|e| format!("Invalid value for {}: {}", #field_name_str, e))?
            }
        } else {
            quote! {
                #field_name: matches.get_one::<String>(#field_name_str)
                    .ok_or_else(|| format!("Missing required argument: {}", #field_name_str))?
                    .parse()
                    .map_err(|e| format!("Invalid value for {}: {}", #field_name_str, e))?
            }
        };
        
        field_parsers.push(parser);
        
        // Generate default values for the Default impl
        let default = if is_optional {
            quote! { #field_name: None }
        } else if let Some(default) = &default_value {
            quote! { #field_name: #default.parse().unwrap_or_default() }
        } else {
            quote! { #field_name: Default::default() }
        };
        
        field_defaults.push(default);
    }
    
    (
        quote! { #(#arg_definitions)* },
        quote! { #(#field_parsers),* },
        quote! { #(#field_defaults),* }
    )
}

// Helper function to parse #[arg(...)] attributes on fields
fn parse_arg_attrs(attrs: &[Attribute], field_name: &str) -> (Option<char>, Option<String>, Option<String>, String, Option<bool>) {
    let mut short = None;
    let mut long = None;
    let mut default_value = None;
    let mut help = format!("The {} argument", field_name);
    let mut required = None;
    
    for attr in attrs {
        if attr.path().is_ident("arg") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens_str = meta_list.tokens.to_string();
                let parts: Vec<&str> = tokens_str.split(',').collect();
                
                for part in parts {
                    let part = part.trim();
                    if part == "short" {
                        short = Some(field_name.chars().next().unwrap_or('s'));
                    } else if part == "long" {
                        long = Some(field_name.to_string());
                    } else if let Some(value) = extract_string_value(part, "short") {
                        short = value.chars().next();
                    } else if let Some(value) = extract_string_value(part, "long") {
                        long = Some(value);
                    } else if let Some(value) = extract_string_value(part, "default_value") {
                        default_value = Some(value);
                    } else if let Some(value) = extract_string_value(part, "help") {
                        help = value;
                    } else if part == "required" {
                        required = Some(true);
                    }
                }
            }
        }
    }
    
    (short, long, default_value, help, required)
}

// Helper function to check if a struct has fields
fn has_fields(data: &syn::Data) -> bool {
    match data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => !fields.named.is_empty(),
            Fields::Unnamed(fields) => !fields.unnamed.is_empty(),
            Fields::Unit => false,
        },
        _ => false,
    }
}

// Helper function to check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

// Helper function to extract string values from attribute tokens
fn extract_string_value(token: &str, key: &str) -> Option<String> {
    let pattern = format!("{} = \"", key);
    if let Some(start) = token.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = token[start..].find('"') {
            return Some(token[start..start + end].to_string());
        }
    }
    None
}

// Helper function to extract array values from attribute tokens
fn extract_array_value(token: &str, key: &str) -> Option<Vec<String>> {
    let pattern = format!("{} = [", key);
    if let Some(start) = token.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = token[start..].find(']') {
            let content = &token[start..start + end];
            let values: Vec<String> = content
                .split(',')
                .map(|s| s.trim().strip_prefix('"').unwrap_or(s).strip_suffix('"').unwrap_or(s).to_string())
                .filter(|s| !s.is_empty())
                .collect();
            return Some(values);
        }
    }
    None
}
