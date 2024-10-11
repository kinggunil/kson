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
serde_kson = "0.2.1"
```

## Example Usage

Here is how you can use the `kson!` macro to build and interact with a JSON-like structure in Rust:

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
    kson!(a["like"]["numbers"][0]["a"] = 777777);
    kson!(a["like"]["numbers"][1]["b"] = 121212);

    println!("{:#?}",a);
    // Output:
    /*
    Object {
        "age": Number(40),
        "country": Array [
            String("Korea"),
            String("Canada"),
        ],
        "like": Object {
            "number": Number(777),
            "numbers": Array [
                Object {
                    "a": Number(777777),
                },
                Object {
                    "b": Number(121212),
                },
            ],
        },
        "name": String("kinggunil"),
        "phone": Object {
            "home": String("031-7**-2440"),
            "office": String("010-28**-3440"),
        },
    }    
     */

    // Access and print the values
    println!("Name: {:?}", kson!(a["name"] : String)); // Output: "kinggunil"
    println!("Age next year: {:?}", kson!(a["age"] : i64) + 1); // Output: 41
    println!("Office phone: {:?}", kson!(a["phone"]["office"] : String)); // Output: "010-28**-3440"
    println!("Home phone: {:?}", kson!(a["phone"]["home"] : String)); // Output: "031-7**-2440"
    println!("First country: {:?}", kson!(a["country"][0] : String)); // Output: "Korea"
    println!("Second country: {:?}", kson!(a["country"][1] : String)); // Output: "Canada"
    println!("number: {:?}", kson!(a["like"]["number"] : i64)); // Output: 777
    println!("number: {:?}", kson!(a["like"]["numbers"][0]["a"] : i64)); // Output: 777
    println!("number: {:?}", kson!(a["like"]["numbers"][1]["b"] : i64)); // Output: 121212

    //////////////////////////////////////////////
    ///////Super flexible type conversion/////////
    //////////////////////////////////////////////
    kson!(b);
    kson!(b["any"] = 36);    // this is i64
    println!("any: {:?}", kson!(b["any"] : String)); // Output: "36" this is string !!
    println!("any: {:?}", kson!(b["any"] : &str)); // Output: "36" this is &str !!

    kson!(b["bee"] = "210316");    // this is String
    println!("bee: {:?}", kson!(b["bee"] : i32)); // Output: 210316 this is i32 !!
    println!("bee: {:?}", kson!(b["bee"] : i64)); // Output: 210316 this is i64 !!
    println!("bee: {:?}", kson!(b["bee"] : f64)); // Output: 210316.0 this is f64 !!


    kson!(c);
    kson!(c[0] = "1");    // this is "String"
    let cc_0=kson!(c[0] : i64);// change to i64
    kson!(c[1] = 3);    // this is i64
    let cc_1=kson!(c[1] : i64);// this is i64


    let dd=cc_0+cc_1; //i64 + i64
    println!("dd: {:?}", dd); // Output: 1
}
```

## License

This project is licensed under the MIT License.