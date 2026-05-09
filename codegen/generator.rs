use std::io;
use std::path::PathBuf;

use convert_case::{Case, Casing};
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::invalid_data;
use crate::parser::BCPMessage;
use crate::parser::Sign;

pub struct BCPBinding {
    pub(crate) file: PathBuf,
    pub(crate) contents: String,
}

#[derive(Debug, Clone, Copy)]
pub enum FieldType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    Bool,
}

impl FieldType {
    pub fn to_tokens(&self) -> TokenStream {
        match self {
            FieldType::U8 => quote! { u8 },
            FieldType::I8 => quote! { i8 },
            FieldType::U16 => quote! { u16 },
            FieldType::I16 => quote! { i16 },
            FieldType::U32 => quote! { u32 },
            FieldType::I32 => quote! { i32 },
            FieldType::Bool => quote! { bool },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EncodingFunction {
    PushBool,
    PushU8,
    PushU16,
    PushU32,
    PushU8NBits(u8),
    PushU16NBits(u8),
    PushU32NBits(u8),
    PushI8,
    PushI16,
    PushI32,
    PushI8NBits(u8),
    PushI16NBits(u8),
    PushI32NBits(u8),
}

impl EncodingFunction {
    pub fn to_call_tokens(&self, value: TokenStream) -> TokenStream {
        match self {
            EncodingFunction::PushBool => quote! {
                bits.push_bool(#value);
            },
            EncodingFunction::PushU8 => quote! {
                bits.push_u8(#value);
            },
            EncodingFunction::PushI8 => quote! {
                bits.push_i8(#value);
            },
            EncodingFunction::PushU16 => quote! {
                bits.push_u16(#value);
            },
            EncodingFunction::PushI16 => quote! {
                bits.push_i16(#value);
            },
            EncodingFunction::PushU32 => quote! {
                bits.push_u32(#value);
            },
            EncodingFunction::PushI32 => quote! {
                bits.push_i32(#value);
            },
            EncodingFunction::PushU8NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    bits.push_u8_n_bits(#value, #len);
                }
            }
            EncodingFunction::PushU16NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    bits.push_u16_n_bits(#value, #len);
                }
            }
            EncodingFunction::PushU32NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    bits.push_u32_n_bits(#value, #len);
                }
            }
            EncodingFunction::PushI8NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    bits.push_i8_n_bits(#value, #len);
                }
            }
            EncodingFunction::PushI16NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    bits.push_i16_n_bits(#value, #len);
                }
            }
            EncodingFunction::PushI32NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    bits.push_i32_n_bits(#value, #len);
                }
            }
        }
    }

    pub fn from_type_and_len(_type: FieldType, len: u8) -> Result<Self, String> {
        if len == 0 || len > 32 {
            return Err(format!(
                "invlid length ({}) to determine encoding function",
                len
            ));
        }
        match _type {
            FieldType::Bool => {
                if len == 1 {
                    Ok(EncodingFunction::PushBool)
                } else {
                    Err("invalid bool length".into())
                }
            }
            FieldType::U8 => {
                if len == 8 {
                    Ok(EncodingFunction::PushU8)
                } else if len < 8 {
                    Ok(EncodingFunction::PushU8NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::I8 => {
                if len == 8 {
                    Ok(EncodingFunction::PushI8)
                } else if len < 8 {
                    Ok(EncodingFunction::PushI8NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::U16 => {
                if len == 16 {
                    Ok(EncodingFunction::PushU16)
                } else if len < 16 {
                    Ok(EncodingFunction::PushU16NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::I16 => {
                if len == 16 {
                    Ok(EncodingFunction::PushI16)
                } else if len < 16 {
                    Ok(EncodingFunction::PushI16NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::U32 => {
                if len == 32 {
                    Ok(EncodingFunction::PushU32)
                } else if len < 32 {
                    Ok(EncodingFunction::PushU32NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::I32 => {
                if len == 32 {
                    Ok(EncodingFunction::PushI32)
                } else if len < 32 {
                    Ok(EncodingFunction::PushI32NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DecodingFunction {
    ReadBool,
    ReadU8,
    ReadU16,
    ReadU32,
    ReadU8NBits(u8),
    ReadU16NBits(u8),
    ReadU32NBits(u8),
    ReadI8,
    ReadI16,
    ReadI32,
    ReadI8NBits(u8),
    ReadI16NBits(u8),
    ReadI32NBits(u8),
}

impl DecodingFunction {
    pub fn to_call_tokens(&self, value: TokenStream) -> TokenStream {
        match self {
            DecodingFunction::ReadBool => quote! {
                let #value = reader.read_bool()?;
            },
            DecodingFunction::ReadU8 => quote! {
                let #value = reader.read_u8()?;
            },
            DecodingFunction::ReadI8 => quote! {
                let #value = reader.read_i8()?;
            },
            DecodingFunction::ReadU16 => quote! {
                let #value = reader.read_u16()?;
            },
            DecodingFunction::ReadI16 => quote! {
                let #value = reader.read_i16()?;
            },
            DecodingFunction::ReadU32 => quote! {
                let #value = reader.read_u32()?;
            },
            DecodingFunction::ReadI32 => quote! {
                let #value = reader.read_i32()?;
            },
            DecodingFunction::ReadU8NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    let #value = reader.read_u8_n_bits(#len)?;
                }
            }
            DecodingFunction::ReadU16NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    let #value = reader.read_u16_n_bits(#len)?;
                }
            }
            DecodingFunction::ReadU32NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    let #value = reader.read_u32_n_bits(#len)?;
                }
            }
            DecodingFunction::ReadI8NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    let #value = reader.read_i8_n_bits(#len)?;
                }
            }
            DecodingFunction::ReadI16NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    let #value = reader.read_i16_n_bits(#len)?;
                }
            }
            DecodingFunction::ReadI32NBits(len) => {
                let len = usize::from(*len);
                quote! {
                    let #value = reader.read_i32_n_bits(#len)?;
                }
            }
        }
    }

    pub fn from_type_and_len(_type: FieldType, len: u8) -> Result<Self, String> {
        if len == 0 || len > 32 {
            return Err(format!(
                "invlid length ({}) to determine encoding function",
                len
            ));
        }
        match _type {
            FieldType::Bool => {
                if len == 1 {
                    Ok(DecodingFunction::ReadBool)
                } else {
                    Err("invalid bool length".into())
                }
            }
            FieldType::U8 => {
                if len == 8 {
                    Ok(DecodingFunction::ReadU8)
                } else if len < 8 {
                    Ok(DecodingFunction::ReadU8NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::I8 => {
                if len == 8 {
                    Ok(DecodingFunction::ReadI8)
                } else if len < 8 {
                    Ok(DecodingFunction::ReadI8NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::U16 => {
                if len == 16 {
                    Ok(DecodingFunction::ReadU16)
                } else if len < 16 {
                    Ok(DecodingFunction::ReadU16NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::I16 => {
                if len == 16 {
                    Ok(DecodingFunction::ReadI16)
                } else if len < 16 {
                    Ok(DecodingFunction::ReadI16NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::U32 => {
                if len == 32 {
                    Ok(DecodingFunction::ReadU32)
                } else if len < 32 {
                    Ok(DecodingFunction::ReadU32NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
            FieldType::I32 => {
                if len == 32 {
                    Ok(DecodingFunction::ReadI32)
                } else if len < 32 {
                    Ok(DecodingFunction::ReadI32NBits(len))
                } else {
                    Err("invlid type and length combination".into())
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BCPFieldBinding {
    pub(crate) name: String,
    pub(crate) _type: FieldType,
    pub(crate) len: u8,
    pub(crate) description: String,
}

fn determine_field_type(len: u8, sign: Sign) -> Result<FieldType, String> {
    if len == 0 {
        return Err("Invalid len (0) for field type".into());
    }

    match sign {
        Sign::Negative => {
            if len == 1 {
                Ok(FieldType::Bool)
            } else if len <= 8 {
                Ok(FieldType::I8)
            } else if len <= 16 {
                Ok(FieldType::I16)
            } else if len <= 32 {
                Ok(FieldType::I32)
            } else {
                Err(format!("Invalid len ({}) for Negative field type", len))
            }
        }
        Sign::Positive => {
            if len == 1 {
                Ok(FieldType::Bool)
            } else if len <= 8 {
                Ok(FieldType::U8)
            } else if len <= 16 {
                Ok(FieldType::U16)
            } else if len <= 32 {
                Ok(FieldType::U32)
            } else {
                Err(format!("Invalid len ({}) for Positive field type", len))
            }
        }
    }
}

pub fn gather_field_bindings(msg: &BCPMessage) -> Result<Vec<BCPFieldBinding>, String> {
    let mut fields: Vec<BCPFieldBinding> = Vec::new();
    for field in &msg.fields {
        fields.push(BCPFieldBinding {
            name: field.name.to_case(Case::Snake).clone(),
            _type: determine_field_type(field.len, field.sign)?,
            len: field.len,
            description: field.description.clone(),
        });
    }
    Ok(fields)
}

#[derive(Debug)]
pub struct EncodingFunctionCall {
    function: EncodingFunction,
    field_name: Ident,
}

impl EncodingFunctionCall {
    pub fn to_tokens(&self) -> TokenStream {
        let field_name = &self.field_name;

        self.function.to_call_tokens(quote! {
            self.#field_name
        })
    }
}

pub struct DecodingFunctionCall {
    function: DecodingFunction,
    field_name: Ident,
}

impl DecodingFunctionCall {
    pub fn to_tokens(&self) -> TokenStream {
        let field_name = &self.field_name;

        self.function.to_call_tokens(quote! {
            #field_name
        })
    }
}
pub fn get_encoding_function_calls(
    fields: &[BCPFieldBinding],
) -> io::Result<Vec<EncodingFunctionCall>> {
    let mut calls: Vec<EncodingFunctionCall> = Vec::new();
    for field in fields {
        calls.push(EncodingFunctionCall {
            function: EncodingFunction::from_type_and_len(field._type, field.len)
                .map_err(invalid_data)?,
            field_name: rust_ident(&field.name.to_case(Case::Snake), "field")?,
        });
    }

    Ok(calls)
}

pub fn get_decoding_function_calls(
    fields: &[BCPFieldBinding],
) -> io::Result<Vec<DecodingFunctionCall>> {
    let mut calls: Vec<DecodingFunctionCall> = Vec::new();
    for field in fields {
        calls.push(DecodingFunctionCall {
            function: DecodingFunction::from_type_and_len(field._type, field.len)
                .map_err(invalid_data)?,
            field_name: rust_ident(&field.name.to_case(Case::Snake), "field")?,
        });
    }

    Ok(calls)
}

pub fn generate_message_binding(msg: &BCPMessage) -> std::io::Result<BCPBinding> {
    let id_const_name = format!("MSG_{}_ID", msg.info.name.to_case(Case::UpperSnake));
    let len_const_name = format!("MSG_{}_LEN", msg.info.name.to_case(Case::UpperSnake));
    let msg_id_const_ident_name = rust_ident(&id_const_name, "message id const")?;
    let msg_len_const_ident_name = rust_ident(&len_const_name, "message len const")?;

    let struct_name = rust_ident(
        &format!("{}Msg", msg.info.name.to_case(Case::Pascal)),
        "message struct",
    )?;

    let msg_id_value = msg.info.id;
    let msg_len_value = msg.info.num_bytes;

    let field_bindings =
        gather_field_bindings(msg).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let field_tokens = field_bindings
        .iter()
        .map(|f| -> io::Result<TokenStream> {
            let name = rust_ident(&f.name.to_case(Case::Snake), "field")?;
            let ty = f._type.to_tokens();

            Ok(quote! {
                pub #name: #ty
            })
        })
        .collect::<io::Result<Vec<_>>>()?;

    let encoding_function_calls = get_encoding_function_calls(&field_bindings)?;

    let encoding_call_tokens = encoding_function_calls
        .iter()
        .map(|call| call.to_tokens())
        .collect::<Vec<_>>();

    let decoding_function_calls = get_decoding_function_calls(&field_bindings)?;

    let decoding_call_tokens = decoding_function_calls
        .iter()
        .map(|call| call.to_tokens())
        .collect::<Vec<_>>();

    let field_names = field_bindings
        .iter()
        .map(|f| rust_ident(&f.name.to_case(Case::Snake), "field"))
        .collect::<io::Result<Vec<_>>>()?;

    let doc_tokens = generate_documentation_tokens(msg, field_bindings);
    let tokens = quote! {
        use crate::codec::{Bits, DecodeError};
        pub const #msg_id_const_ident_name: u16 = #msg_id_value;
        pub const #msg_len_const_ident_name: u8 = #msg_len_value;
        #(#doc_tokens)*

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct #struct_name {
            #(#field_tokens,)*
        }

        impl #struct_name {
            pub fn encode(&self) -> Bits {
                let mut bits = Bits::new();
                bits.push_header(#msg_id_const_ident_name, #msg_len_const_ident_name);
                #(#encoding_call_tokens)*
                bits.append_crc();
                bits
            }

            pub fn decode(&self, bits: &Bits) -> Result<Self, DecodeError> {
                let mut reader = bits.reader();

                let id = reader.read_id()?;
                reader.validate_id(id, #msg_id_const_ident_name)?;

                let len = reader.read_len()?;
                reader.validate_len(len, #msg_len_const_ident_name)?;

                #(#decoding_call_tokens)*

                Ok(Self{
                    #(#field_names,)*
                })
            }
        }
    };

    let file = PathBuf::from(format!("msg_{}.rs", msg.info.name.to_case(Case::Snake)));

    Ok(BCPBinding {
        file,
        contents: render_tokens(tokens)?,
    })
}

fn generate_documentation_tokens(
    msg: &BCPMessage,
    field_bindings: Vec<BCPFieldBinding>,
) -> Vec<TokenStream> {
    let blank = "".to_string();
    let description_heading = format!(" ## Description");
    let description = format!(" {}", msg.info.description);

    let field_docs: Vec<TokenStream> = std::iter::once(quote! {
        #[doc = #description_heading]
        #[doc = #description]
        #[doc = #blank]
    })
    .chain(field_bindings.iter().map(|field| {
        let name = format!("- [`{}`]: {}", field.name, field.description);
        quote! {
            #[doc = #name]
        }
    }))
    .collect();

    field_docs
}

fn render_tokens(tokens: TokenStream) -> io::Result<String> {
    let syntax_tree = syn::parse2::<syn::File>(tokens)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(prettyplease::unparse(&syntax_tree))
}

fn rust_ident(name: &str, context: &str) -> io::Result<Ident> {
    let candidate = if RUST_KEYWORDS.contains(&name) {
        format!("r#{name}")
    } else {
        name.to_string()
    };

    syn::parse_str::<Ident>(&candidate).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{context} name `{name}` is not a valid Rust identifier: {e}"),
        )
    })
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];
