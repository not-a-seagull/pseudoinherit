# pseudoinherit

Creates a fake form of inheritance in Rust.

**IMPORTANT NOTE:** This project is nowhere near ready for use in testing environments,
let alone prod. Please don't use this until it is ready.

One of my main issues with Rust is its lack of inheritance. 99% of the places where
inheritance could be used can generally be covered by Rust's strong trait system and
generic support. The purpose of this crate is to cover the remaining 1%.

The following is what I would like a program using pseudoinherit to look like:

```rust
use pseudoinherit::{Derives, pseudo};

pseudo! {
    struct Animal {
        pub name: String;
    }

    impl Animal {
        fn new(name: String) -> Animal {
            create_item!(name)
        }

        fn print_name(&self) {
            println!("Hello! My name is {}.", self.name);
        }
    }

    struct Dog : Animal {
        pub color: String;
    }

    impl Dog {
        fn new(name: String, color: String) -> Dog {
            create_item!(super::new(name), color);
        }

        fn print_name(&self) {
            println!("Bark bark! My name is {} and my color is {}.", self.name, self.color);
        }
    }
}

fn describe_animal<T: Derives<Animal>>(animal: T) {
    animal.print_name();
}

fn main() {
   let animal = Animal::new("Froggy");
   let dog = Dog::new("Fido", "brown");

   describe_animal(animal);
   describe_animal(dog);
}
```
