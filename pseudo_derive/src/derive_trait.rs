/* 
 * pseudo_derive/src/derive_trait.rs - Implementation for Derivable.
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

pub fn implement_derive(sd_name: &Ident, sdp_name: &Ident, sdt_name: &Ident) -> TokenStream {
    quote! {
        impl pseudoinherit::Derivable for #sd_name {
            type Properties = #sdp_name;

            fn get_props(&self) -> &#sdp_name {
                &self.properties
            }

            fn get_props_mut(&mut self) -> &mut #sdp_name {
                &mut self.properties
            }
        }

        impl<T: pseudoinherit::Derives<#sd_name>> #sdt_name for T {
            fn get_props(&self) -> &#sdp_name {
                pseudoinherit::Derives::<#sd_name>::get_props(self)
            }

            fn get_props_mut(&mut self) -> &mut #sdp_name {
                pseudoinherit::Derives::<#sd_name>::get_props_mut(self)
            }
        }
    } 
}
