/* 
 * pseudo_derive/src/base_derive.rs - Implement Derives to derive from the bases.
 * pseudoinherit - Fake inheritance in Rust.
 * 
 * This software may be licensed under the terms of either the MIT License
 * (can be found in LICENSE-MIT) or the Apache 2.0 License (can be found in
 * LICENSE-APACHE), at your option. Please consult either license file for
 * more information.
 */

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn implement_base_class(sd_name: &Ident, bc_name: &Ident, bcp_name: &Ident) -> TokenStream {
    quote! {
        impl<T: pseudoinherit::Derives<#sd_name>> pseudoinherit::Derives<#bc_name> for T {
            fn get_props(&self) -> &#bcp_name {
                &pseudoinherit::Derives::<#sd_name>::get_props(self).base_class_props
            }

            fn get_props_mut(&mut self) -> &mut #bcp_name {
                &mut pseudoinherit::Derives::<#sd_name>::get_props_mut(self).base_class_props
            }
        }
    }
}
