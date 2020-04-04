/* 
 * pseudo_derive/src/lib.rs - Define the pseudo! macro, which creates "fake inheritance".
 * pseudoinherit - Fake inheritance in Rust.
 * 
 * This software may be licensed under the terms of either the MIT License
 * (can be found in LICENSE-MIT) or the Apache 2.0 License (can be found in
 * LICENSE-APACHE), at your option. Please consult either license file for
 * more information.
 */

mod base_trait;
mod base_derive;
mod derive_trait;
mod structure;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use structure::DeriveStruct;
use syn::{token::Crate, Field, parse_macro_input, Visibility, VisCrate};

/// Takes one struct (and, optionally, one impl block) within and creates an inheritable class.
#[proc_macro]
pub fn pseudo(ts: TokenStream) -> TokenStream {
    let s_defn = parse_macro_input!(ts as DeriveStruct);
   
    // various useful variables 
    let sd_name = &s_defn.name;
    let sdp_name = format_ident!("{}Props", sd_name);
    let sd_vis = &s_defn.vis;
    let sd_fields = &s_defn.fields;
    let base_class = &s_defn.base_class;
    let is_derived = base_class.is_some();
    let base_class_name = if is_derived { Some(&base_class.as_ref().unwrap().1) } else { None };
    let bcp_name = if is_derived { Some(format_ident!("{}Props", base_class_name.unwrap())) } else { None };
    let sdt_name = format_ident!("{}Base", sd_name);

    // add a reference to the base class's properties, if necessary
    let sdp_base_class_ref = if (&s_defn.base_class).is_some() {
        quote! {
            pub(crate) base_class_props: #bcp_name, 
        }
    } else {
        quote! {}
    };

    let mut sdp_fields = sd_fields.clone();
    sdp_fields.iter_mut().for_each(|f| f.vis = Visibility::Crate(
        VisCrate { crate_token: Crate { span: Span::call_site() } }
    ));
    let sdp_fields: Vec<&Field> = sdp_fields.iter().collect();

    // the properties of the current class
    let sdp = quote! {
        #sd_vis #sdp_name {
            #(#sdp_fields),*
            ,
            #sdp_base_class_ref
        } 
    };

    // an instance of the class
    let sd_struct = quote! {
        #sd_vis #sd_name {
            properties: #sdp_name
        }
    }; 

    // the basic trait for the class
    let sd_trait = base_trait::base_trait(sd_vis, &sdt_name, &sdp_name, sd_fields);
    
    // implement the Derivables for this item
    let sd_derive = derive_trait::implement_derive(sd_name, &sdp_name, &sdt_name);

    // if needed, implement for the base class
    let sd_base_derive = if is_derived {
        Some(base_derive::implement_base_class(sd_name, base_class_name.unwrap(), &bcp_name.unwrap()))
    } else { 
        None 
    };

    let tokens = quote! {
        #sdp

        #sd_struct
  
        #sd_trait
 
        #sd_derive
  
        #sd_base_derive
    };

    tokens.into()
}
