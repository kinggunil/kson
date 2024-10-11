# serde_kson Macro

`serde_kson` is a Rust macro that simplifies the process of building and managing JSON-like data structures. It allows you to dynamically create and manipulate nested JSON objects and arrays using a simple and intuitive syntax.

## Features

- Easily create nested JSON structures.
- Supports both object and array syntax.
- Convenient access and update operations.
- Built on top of `serde_json` for seamless integration.

## Dependencies

To use `serde_kson`, make sure your `Cargo.toml` includes the following dependencies:

```toml
[dependencies]
serde_json = "1.0"
serde_kson = "0.1.0"



```rust
use serde_kson::kson; // Import the kson macro

fn main() {
    // Create a new JSON-like structure
    kson!(a);
    
    // Set values for the JSON object
    kson!(a["name"] = "kinggunil");
    kson!(a["age"] = 40);
    kson!(a["phone"]["office"] = "010-28**-3440");
    kson!(a["phone"]["home"] = "031-7**-2440");
    kson!(a["country"][0] = "Korea");
    kson!(a["country"][1] = "Canada");
    kson!(a["like"]["number"] = 777);

    // Access and print the values
    println!("Name: {:?}", kson!(a["name"] : String)); // Output: "kinggunil"
    println!("Age next year: {:?}", kson!(a["age"] : i64) + 1); // Output: 41
    println!("Office phone: {:?}", kson!(a["phone"]["office"] : String)); // Output: "010-28**-3440"
    println!("Home phone: {:?}", kson!(a["phone"]["home"] : String)); // Output: "031-7**-2440"
    println!("First country: {:?}", kson!(a["country"][0] : String)); // Output: "Korea"
    println!("Second country: {:?}", kson!(a["country"][1] : String)); // Output: "Canada"
    println!("Favorite number: {:?}", kson!(a["like"]["number"] : i64)); // Output: 777
}
