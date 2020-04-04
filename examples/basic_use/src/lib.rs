/* 
 * examples/basic_use/src/lib.rs - Test to see if basic functionality works.
 * pseudoinherit - Fake inheritance in Rust.
 * 
 * This software may be licensed under the terms of either the MIT License
 * (can be found in LICENSE-MIT) or the Apache 2.0 License (can be found in
 * LICENSE-APACHE), at your option. Please consult either license file for
 * more information.
 */

use pseudoinherit::pseudo;

pseudo! {
    pub struct A {
        field1: i32,
        field2: i64,
    }
}
