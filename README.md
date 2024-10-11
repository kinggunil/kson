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
```

## Example Usage

Here is how you can use the `kson!` macro to build and interact with a JSON-like structure in Rust:

```rust
use serde_kson::kson; // Import the kson macro

fn main() {
    // Create a new JSON-like structure
    let mut a = kson!({});
    
    // Set values for the JSON object
    a["name"] = "kinggunil".into();
    a["age"] = 40.into();
    a["phone"]["office"] = "010-28**-3440".into();
    a["phone"]["home"] = "031-7**-2440".into();
    a["country"][0] = "Korea".into();
    a["country"][1] = "Canada".into();
    a["like"]["number"] = 777.into();

    // Access and print the values
    println!("Name: {:?}", a["name"].as_str().unwrap()); // Output: "kinggunil"
    println!("Age next year: {:?}", a["age"].as_i64().unwrap() + 1); // Output: 41
    println!("Office phone: {:?}", a["phone"]["office"].as_str().unwrap()); // Output: "010-28**-3440"
    println!("Home phone: {:?}", a["phone"]["home"].as_str().unwrap()); // Output: "031-7**-2440"
    println!("First country: {:?}", a["country"][0].as_str().unwrap()); // Output: "Korea"
    println!("Second country: {:?}", a["country"][1].as_str().unwrap()); // Output: "Canada"
    println!("Favorite number: {:?}", a["like"]["number"].as_i64().unwrap()); // Output: 777
}
```

## License

This project is licensed under the MIT License.
