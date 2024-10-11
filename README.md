# serde_kson (Macro)

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
serde_kson = "0.3.3" #for macro
serde_json = "1.0" #for macro
rand = "0.8.5" #for functions
chrono = "0.4.38" #for functions
```

## Example Usage(macro)

Here is how you can use the `kson!` macro to build and interact with a JSON-like structure in Rust:

```rust
use serde_kson::*; // Import the kson macro


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

// Access and print the values
println!("Name: {:?}", kson!(a["name"] : String)); // Output(String): "kinggunil"
println!("Age next year: {:?}", kson!(a["age"] : i64) + 1); // Output(i64): 41
println!("Office phone: {:?}", kson!(a["phone"]["office"] : String)); // Output(String): "010-28**-3440"
println!("Home phone: {:?}", kson!(a["phone"]["home"] : String)); // Output(String): "031-7**-2440"
println!("country01 : {:?}", kson!(a["country"][0] : String)); // Output(String): "Korea"
println!("country02 : {:?}", kson!(a["country"][1] : String)); // Output(String): "Canada"
println!("number: {:?}", kson!(a["like"]["number"] : i64)); // Output(i64): 777 
println!("number: {:?}", kson!(a["like"]["numbers"][0]["a"] : i64)); // Output(i64): 777
println!("number: {:?}", kson!(a["like"]["numbers"][1]["b"] : i64)); // Output(i64): 121212
println!("{:#?}" , a);
// Output:
/*
Object {
    "name": String("kinggunil"),
    "age": Number(40),
    "phone": Object {
        "office": String("010-28**-3440"),
        "home": String("031-7**-2440"),
    },
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
}
*/


/////// very easy flexible type conversion/////////

kson!(b); // Create a new JSON-like structure
kson!(b["any"] = 36); // : i64
println!("any: {:?}", kson!(b["any"] : String)); // Output(String): "36"
println!("any: {:?}", kson!(b["any"] : &str)); // Output(&str): "36"

kson!(b["bee"] = "210316");    // this is String
println!("bee: {:?}", kson!(b["bee"] : i32)); // Output(i32): 210316 
println!("bee: {:?}", kson!(b["bee"] : i64) + 9000000); // Output(i64): 9210316 
println!("bee: {:?}", kson!(b["bee"] : f64) + 0.77); // Output(f64): 210316.77 


kson!(c); // Create a new JSON-like structure

kson!(c[0] = "1");    // this is "String"
let cc_0=kson!(c[0] : i64);// changed to i64

kson!(c[1] = 3);    // this is i64
let cc_1=kson!(c[1] : i64);// this is i64

let dd=cc_0+cc_1; //i64 + i64
println!("dd: {:?}", dd); // Output: 1

```

---
---

# serde_kson (Functions)

The `serde_kson` library provides a set of utility functions to perform various operations, such as generating random numbers, sleeping the current thread, formatting numbers, and converting UNIX timestamps to human-readable dates.

## Features

- `kson_rand`: Generate a random integer within a specified range.
- `kson_sleep`: Pause execution for a specified number of seconds.
- `kson_time`: Get the current Unix time in seconds.
- `kson_microtime`: Get the current Unix time in microseconds.
- `kson_number_format`: Format numbers with a specified number of decimal places and thousands separators.
- `kson_datetime`: Convert Unix timestamps (seconds or microseconds) to a formatted local date and time string (`Y-m-d H:i:s`).

## Example Usage

Below are the descriptions of the functions and how to use them in your Rust code:

```rust
use serde_kson::*;

// kson_rand: Generates a random number between `min` and `max` (inclusive).
// Returns: i64
let random_num = kson_rand(1, 100); 
println!("Random number: {}", random_num); // Output: 79

let another_random_num = kson_rand(-500, 10); 
println!("Another random number: {}", another_random_num); // Output: -324

// kson_sleep: Suspends the current thread for the specified number of seconds.
// Returns: ()
kson_sleep(2.5);  // Sleeps for 2.5 seconds
kson_sleep(0.005);  // Sleeps for 0.005 second

// kson_time: Returns the current UNIX time in seconds.
// Returns: u64
let unix_time = kson_time(); // Outputs: 1728663849
println!("Current UNIX time: {}", unix_time);

// kson_microtime: Returns the current UNIX time in microseconds.
// Returns: u64
let micro_time = kson_microtime(); // Outputs: 1728663849000
println!("Current UNIX time (microseconds): {}", micro_time);

// kson_number_format: Formats a number with the specified number of decimal places and inserts commas to separate thousands.
// Returns: String
let formatted = kson_number_format(1234567.89123, 2);
println!("Formatted number: {}", formatted);  // Outputs: "1,234,567.89"

let another_formatted = kson_number_format(987654321.12345, 3);
println!("Another formatted number: {}", another_formatted);  // Outputs: "987,654,321.123"

// kson_datetime: Converts a UNIX timestamp (in seconds or microseconds) to a formatted string in the local timezone.
// Returns: String
let datetime = kson_datetime(1694444030);
println!("Local datetime: {}", datetime);  // Outputs: "2024-10-11 15:13:50"

let datetime = kson_datetime(kson_time());
println!("Local datetime: {}", datetime);  // Outputs: "2024-10-11 15:30:00" <=current

let another_datetime = kson_datetime(kson_microtime());
println!("Another local datetime: {}", another_datetime);  // Outputs: "2024-10-11 15:30:00" <=current


///////// Usage : macro and funtions together //////////

kson!(thing);// Create a new JSON-like structure
kson!(thing["unixTime"] = kson_time());    // this is String
kson!(thing["now_time"] = kson_datetime(kson_microtime()));    // this is String
kson!(thing["thing_r"] = kson_rand(1, 99999999));    // this is String
kson!(thing["numberFormat"] = kson_number_format(123498.75456789, 5));    // this is i64

println!("{:#?}",thing);
// Output:
/*
Object {
    "now_time": String("2024-10-12 01:59:38"),
    "numberFormat": String("123,498.75457"),
    "thing": Number(7435865),
    "unixTime": Number(1728665978),
}
*/


```



## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

Feel free to use the `serde_kson` library in your projects to make dealing with various utility tasks easier and more convenient.