/* 
 * pseudo_derive/src/structure.rs - A structure definition for the pseudo! macro.
 * pseudoinherit - Fake inheritance in Rust.
 * 
 * This software may be licensed under the terms of either the MIT License
 * (can be found in LICENSE-MIT) or the Apache 2.0 License (can be found in
 * LICENSE-APACHE), at your option. Please consult either license file for
 * more information.
 */

use syn::{
  parse::{Parse, ParseStream, Result},
  Attribute, Fields, Ident, Token, Visibility,
};

/// A struct that can hold inheritance.
pub struct DeriveStruct {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub struct_token: Token![struct],
    pub name: Ident,
    pub base_class: Option<(Token![:], Ident)>,
    pub fields: Fields,
}

impl Parse for DeriveStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            base_class: match input.parse() {
                Ok(i) => Some((i, input.parse()?)),
                Err(_) => None,
            },
            fields: Fields::Named(input.parse()?),
        })
    }
}
