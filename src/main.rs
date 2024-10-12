use serde_kson::*;

fn main() {

    
    kson!(a);
    // Create a new JSON-like structure

    /////////////////////////
    ////// Set values///////
    /////////////////////////

    kson!( a["name"] = "kinggunil" ); 
    //set a["name"] = "kinggunil"

    kson!( a["age"] = 40 );
    //set a["age"] = 40

    kson!( a["phone"]["office"] = "010-28**-3440" );
    //set a["phone"]["office"] = "010-28**-3440"

    kson!( a["phone"]["home"] = "031-7**-2440" );
    //set a["phone"]["home"] = "031-7**-2440"

    kson!( a["country"][0] = "Korea" );
    //set a["country"][0] = "Korea"

    kson!( a["country"][1] = "Canada" );
    //set a["country"][1] = "Canada"

    kson!( a["like"]["number"] = 777 );
    //set a["like"]["number"] = 777

    kson!( a["like"]["numbers"][0]["a"] = 777777 );
    //set a["like"]["numbers"][0]["a"] = 777777

    kson!( a["like"]["numbers"][1]["b"] = 121212 );
    //set a["like"]["numbers"][1]["b"] = 121212


    ////////////////////////////////
    // Access and print the values//
    ////////////////////////////////

    println!("{:?}", kson!( a["name"] => String)  ); 
    // Output(String): "kinggunil"

    println!("{:?}", kson!( a["age"] => i64) + 1 ); 
    // Output(i64): 41

    println!("{:?}", kson!( a["phone"]["office"] => String) ); 
    // Output(String): "010-28**-3440"

    println!("{:?}", kson!( a["phone"]["home"] => String) ); 
    // Output(String): "031-7**-2440"

    println!("{:?}", kson!( a["country"][0] => String) ); 
    // Output(String): "Korea"

    println!("{:?}", kson!( a["country"][1] => String) ); 
    // Output(String): "Canada"

    println!("{:?}", kson!( a["like"]["number"] => i64) ); 
    // Output(i64): 777

    println!("{:?}", kson!( a["like"]["numbers"][0]["a"] => i64) ); 
    // Output(i64): 777

    println!("{:?}", kson!( a["like"]["numbers"][1]["b"] => i64) );
    // Output(i64): 121212

    println!("{:#?}", a );
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

    kson!(b); 
    // Create a new JSON-like structure

    kson!( b["any"] = 36 );
    // set b["any"] = 36

    println!("any: {:?}", kson!( b["any"] => String) ); 
    // Output(String): "36"

    kson!( b["bee"] = "210316" ); 
    // set b["bee"] = "210316"

    println!("bee: {:?}", kson!( b["bee"] => i32) ); 
    // Output(i32): 210316

    println!("bee: {:?}", kson!( b["bee"] => i64) + 9000000 ); 
    // Output(i64): 9210316

    println!("bee: {:?}", kson!( b["bee"] => f64) + 0.77 ); 
    // Output(f64): 210316.77

    kson!(c); 
    // Create a new JSON-like structure

    kson!( c[0] = "55" ); 
    // set c[0] = "1"

    let cc_0 = kson!( c[0] => i64 ); 
    // Output(i64): 55

    kson!( c[1] = 33 );
    // set c[1] = 33

    let cc_1 = kson!( c[1] => i64 );
    // Output(i64): 33

    let dd = cc_0 + cc_1; 
    // i64 + i64

    println!("dd: {:?}", dd );
    // Output(i64): 88


    ////// more flexible type conversion/////////

    println!("{:#?}", kson!( 77 => String) + "99" );
    // Output(String): "7799"

    println!("{:#?}", kson!( "77" => String) + "99" );
    // Output(String): "7799"

    println!("{:#?}", kson!( "50" => i64)+99 );     
    // Output(i64): 149

    println!("{:#?}", kson!( "50" => i32)+99 );     
    // Output(i32): 149

    println!("{:#?}", kson!( "50" => f32)+99.0 );     
    // Output(f32): 149.0

    println!("{:#?}", kson!( "50" => f64)+99.0 );     
    // Output(f64): 149.0

    println!("{:#?}", kson!( 50.0 => f32)+99.0 );     
    // Output(f32): 149.0

    println!("{:#?}", kson!( 50.0 => i64)+99 );     
    // Output(i64): 149


    // kson_rand: Generates a random number between `min` and `max` (inclusive).
    let random_num = kson_rand(1, 100 ); 
    println!("Random number: {}", random_num ); 
    // Output(i64): 79

    let another_random_num = kson_rand(-500, 10 ); 
    println!("Another random number: {}", another_random_num ); 
    // Output(i64): -324

    // kson_sleep: Suspends the current thread for the specified number of seconds.
    kson_sleep(1.00000001 );  
    // Sleeps for 2.5 seconds

    kson_sleep(0.005 );  
    // Sleeps for 0.005 second

    // kson_time: Returns the current UNIX time in seconds.
    let unix_time = kson_time(); 
    println!("Current UNIX time: {}", unix_time );
    // Outputs(u64): 1728663849

    // kson_microtime: Returns the current UNIX time in microseconds.
    let micro_time = kson_microtime(); 
    println!("Current UNIX time (microseconds): {}", micro_time );
    // Outputs(u64): 1728663849000

    // kson_number_format: Formats a number with the specified number of decimal places and inserts commas to separate thousands.
    let formatted = kson_number_format(1234567.89123, 2 );
    println!("Formatted number: {}", formatted );  
    // Outputs(String): "1,234,567.89"

    let another_formatted = kson_number_format(987654321.12345, 3 );
    println!("Another formatted number: {}", another_formatted );
    // Outputs(String): "987,654,321.123"

    // kson_datetime: Converts a UNIX timestamp (in seconds or microseconds) to a formatted string in the local timezone.

    let datetime = kson_datetime(1694444030 );
    println!("Local datetime: {}", datetime );  
    // Outputs(String): "2024-10-11 15:13:50"

    let datetime = kson_datetime(kson_time() );
    println!("Local datetime: {}", datetime );  
    // Outputs(String): "2024-10-11 15:30:00" <=current

    let another_datetime = kson_datetime(kson_microtime() );
    println!("Another local datetime: {}", another_datetime );  
    // Outputs(String): "2024-10-11 15:30:00" <=current


    ///////// Usage : macro and funtions together //////////
    kson!(thing);
    // Create a new JSON-like structure

    kson!( thing["unixTime"] = kson_time() );
    // set thing["unixTime"] = kson_time();

    kson!( thing["now_time"] = kson_datetime(kson_microtime()) );
    // set thing["now_time"] = kson_datetime(kson_microtime());

    kson!( thing["thing_r"] = kson_rand(1, 99999999) );
    // set thing["thing_r"] = kson_rand(1, 99999999);

    kson!( thing["numberFormat"] = kson_number_format(123498.75456789, 5) );
    // set thing["numberFormat"] = kson_number_format(123498.75456789, 5);

    println!("{:#?}",thing );
    // Output:
    /*
    Object {
        "unixTime": Number(1728665978),
        "now_time": String("2024-10-12 01:59:38"),
        "thing_r": Number(7435865),
        "numberFormat": String("123,498.75457"),
    }
    */

    kson();




}
