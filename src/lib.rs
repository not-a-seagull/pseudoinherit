/* 
 * src/lib.rs - Library root
 * psuedoinherit - Fake inheritance in Rust.
 * 
 * This software may be licensed under the terms of either the MIT License
 * (can be found in LICENSE-MIT) or the Apache 2.0 License (can be found in
 * LICENSE-APACHE), at your option. Please consult either license file for
 * more information.
 */

/// Indicates that a type can be derived from.
pub trait Derivable {
    /// The type of the properties of this item.
    type Properties;
    /// Get the properties of this item.
    fn get_props(&self) -> &Self::Properties;
    /// Get a mutable reference to the properties of this item.
    fn get_props_mut(&mut self) -> &mut Self::Properties;
}

/// Indicates that a struct is dervied from another struct.
pub trait Derives<T: Derivable> {
    /// Get the properties of this item.
    fn get_props(&self) -> &T::Properties;
    /// Get a mutable reference to the properties of this item.
    fn get_props_mut(&mut self) -> &mut T::Properties;
}

impl<T: Derivable> Derives<T> for T {
    fn get_props(&self) -> &T::Properties {
        Derivable::get_props(self)
    }

    fn get_props_mut(&mut self) -> &mut T::Properties {
        Derivable::get_props_mut(self)
    }
}

pub use psuedo_derive::*;
