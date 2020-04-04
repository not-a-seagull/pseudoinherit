/* 
 * pseudo_derive/src/base_trait.rs - Create the base trait for inheritance from a Struct. 
 * pseudoinherit - Fake inheritance in Rust.
 * 
 * This software may be licensed under the terms of either the MIT License
 * (can be found in LICENSE-MIT) or the Apache 2.0 License (can be found in
 * LICENSE-APACHE), at your option. Please consult either license file for
 * more information.
 */

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Fields, Ident, Visibility};

pub fn base_trait(vis: &Visibility, bt_name: &Ident, props_name: &Ident, fields: &Fields) -> TokenStream {
    let g_and_s: Vec<TokenStream> = fields.iter().map(|f| getter_and_setter(f)).collect();

    quote! {
        #vis trait #bt_name {
            fn get_props(&self) -> &#props_name;
            fn get_props_mut(&mut self) -> &mut #props_name;

            #(#g_and_s)*
        }
    }
}

/// Create a getter and setter for a particular field.
fn getter_and_setter(field: &Field) -> TokenStream {
    let f_ident = &field.ident.as_ref().unwrap();
    let f_type = &field.ty;
    let f_vis = &field.vis;

    let getter_ident = format_ident!("get_{}", f_ident);
    let setter_ident = format_ident!("set_{}", f_ident);

    quote! {
        #f_vis fn #getter_ident (&self) -> &#f_type {
            &self.get_props().#f_ident
        }

        #f_vis fn #setter_ident (&mut self, val: #f_type) {
            self.get_props_mut().#f_ident = val;
        }
    }
}
