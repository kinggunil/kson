// src/lib.rs










#[macro_export]
macro_rules! kson {
    // 변수를 초기화할 때 사용
    ($var:ident) => {
        let mut $var = serde_json::json!({});
    };

    // 키-값 쌍을 설정할 때 사용
    ($var:ident $( [ $key:expr ] )+ = $value:expr ) => {{
        let mut temp = &mut $var;
        $(
            let key = $key;
            temp = {
                if let Some(index) = key.to_string().parse::<usize>().ok() {
                    if !temp.is_array() {
                        *temp = serde_json::json!([]);
                    }
                    let arr = temp.as_array_mut().unwrap();
                    if arr.len() <= index {
                        arr.resize(index + 1, serde_json::Value::Null);
                    }
                    &mut arr[index]
                } else {
                    let key_str = key.to_string();
                    if !temp.is_object() {
                        *temp = serde_json::json!({});
                    }
                    temp.as_object_mut().unwrap()
                        .entry(key_str)
                        .or_insert_with(|| serde_json::Value::Null)
                }
            };
        )*
        *temp = serde_json::json!($value);
    }};

    // 문자열 값을 가져올 때 사용
    ($var:ident $( [ $key:expr ] )+ : &str) => {{
        let mut temp = &$var;
        $(
            let key = $key;
            temp = {
                if let Some(index) = key.to_string().parse::<usize>().ok() {
                    if let Some(arr) = temp.as_array() {
                        arr.get(index).unwrap_or(&serde_json::Value::Null)
                    } else {
                        &serde_json::Value::Null
                    }
                } else {
                    if let Some(obj) = temp.as_object() {
                        obj.get(&key.to_string()).unwrap_or(&serde_json::Value::Null)
                    } else {
                        &serde_json::Value::Null
                    }
                }
            };
        )*
        // 숫자일 경우 문자열로 변환하여 반환
        if let Some(str_value) = temp.as_str() {
            str_value
        } else if let Some(num_value) = temp.as_i64() {
            Box::leak(Box::new(num_value.to_string()))
        } else if let Some(num_value) = temp.as_f64() {
            Box::leak(Box::new(num_value.to_string()))
        } else {
            panic!("Cannot convert to type '&str'.")
        }
    }};

    // 특정 타입으로 값을 가져올 때 사용
    ($var:ident $( [ $key:expr ] ) + : $t:ty) => {{
        let mut temp = &$var;
        $(
            let key = $key;
            temp = {
                if let Some(index) = key.to_string().parse::<usize>().ok() {
                    if let Some(arr) = temp.as_array() {
                        arr.get(index).unwrap_or(&serde_json::Value::Null)
                    } else {
                        &serde_json::Value::Null
                    }
                } else {
                    if let Some(obj) = temp.as_object() {
                        obj.get(&key.to_string()).unwrap_or(&serde_json::Value::Null)
                    } else {
                        &serde_json::Value::Null
                    }
                }
            };
        )*


        


        



        // 자동 타입 변환 시도
        serde_json::from_value::<$t>(temp.clone()).unwrap_or_else(|_| {
            if let Some(str_value) = temp.as_str() {
                // 문자열로 저장된 숫자를 파싱
                if let Ok(parsed_value) = str_value.parse::<$t>() {
                    parsed_value
                } else {
                    panic!("Failed to convert the value to type '{}'.", stringify!($t))
                }
            } else if let Some(num_value) = temp.as_f64() {
                // 숫자를 문자열로 변환하는 경우
                if stringify!($t) == "String" || stringify!($t) == "&str" {
                    num_value.to_string().parse::<$t>().unwrap()
                } else {
                    panic!("Failed to convert the value to type '{}'.", stringify!($t))
                }
            } else {
                panic!("Failed to convert the value to type '{}'.", stringify!($t))
            }
        })
    }};

    // 지정된 표현식을 원하는 타입으로 변환
    ($expr:expr => String) => {{
        let value = serde_json::json!($expr);
        if let Some(s) = value.as_str() {
            s.to_string()
        } else if let Some(n) = value.as_f64() {
            n.to_string()
        } else {
            panic!("Failed to convert the value to String.")
        }
    }};
    ($expr:expr => $t:ty) => {{
        let value = serde_json::json!($expr);
        serde_json::from_value::<$t>(value.clone()).unwrap_or_else(|_| {
            if let Some(s) = value.as_str() {
                // 문자열을 원하는 타입으로 파싱 시도
                s.parse::<$t>().unwrap_or_else(|_| {
                    panic!("'{}'를 타입 '{}'로 파싱할 수 없습니다.", s, stringify!($t))
                })
            } else if let Some(n) = value.as_f64() {
                // 숫자를 원하는 타입으로 캐스팅 시도
                n as $t
            } else {
                panic!("Failed to convert the value to type '{}'.", stringify!($t))
            }
        })
    }};




}


use rand::Rng;
pub fn kson_rand(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

use std::thread;
use std::time::Duration;
pub fn kson_sleep(seconds: f64) {
    let duration = Duration::from_secs_f64(seconds);
    thread::sleep(duration);
}

use std::time::{SystemTime, UNIX_EPOCH};
/// 현재 유닉스 타임을 초 단위로 반환
pub fn kson_time() -> u64 {
    // 현재 시간
    let start = SystemTime::now();

    // 유닉스 에포크 이후의 경과 시간을 초 단위로 변환
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

/// 현재 유닉스 타임을 마이크로초 단위로 반환
pub fn kson_microtime() -> u64 {
    // 현재 시간
    let start = SystemTime::now();
    // 유닉스 에포크 이후의 경과 시간을 마이크로초 단위로 변환하고 u64로 반환
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as u64
}

pub fn kson_number_format<T: ToString>(num: T, decimals: usize) -> String {
    // 숫자를 문자열로 변환하고 형식화
    let formatted = format!("{:.*}", decimals, num.to_string().parse::<f64>().unwrap());
    let parts: Vec<&str> = formatted.split('.').collect();
    let int_part = parts[0]
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();
    if parts.len() > 1 {
        format!("{}.{}", int_part, parts[1])
    } else {
        int_part
    }
}

use chrono::{Local, TimeZone, Utc};
/// 유닉스 타임(초 또는 마이크로초)을 로컬 시간으로 변환하여 "Y-m-d H:i:s" 형식으로 반환
pub fn kson_datetime(unix_time: u64) -> String {
    // 유닉스 타임이 마이크로초인지 초 단위인지 확인
    let datetime = if unix_time > 1_000_000_000_000 {
        // 마이크로초 단위인 경우
        let secs = (unix_time / 1_000_000) as i64;
        let nanos = ((unix_time % 1_000_000) as u32) * 1_000;
        Utc.timestamp_opt(secs, nanos).unwrap()
    } else {
        // 초 단위인 경우
        Utc.timestamp_opt(unix_time as i64, 0).unwrap()
    };

    // 로컬 타임존으로 변환
    let local_time = datetime.with_timezone(&Local);

    // "Y-m-d H:i:s" 형식으로 변환
    local_time.format("%Y-%m-%d %H:%M:%S").to_string()
}


use std::time::Instant;
static mut START_TIME: Option<Instant> = None;

pub fn kson_tic() {
    unsafe {
        START_TIME = Some(Instant::now());
    }
}

pub fn kson_toc() {
    unsafe {
        if let Some(start) = START_TIME {
            let duration = start.elapsed();
            println!("\nElapsed time: {:?}\n", duration);
        } else {
            println!("Error: kson_tic() was not called.");
        }
        START_TIME = None;  // Reset START_TIME
    }
}



pub fn kson() {
    let blue = "\x1b[34m";
    let reset = "\x1b[0m";

    println!("{}============================{}", blue, reset);
    println!("{}kson function list{}", blue, reset);
    println!("{}============================{}", blue, reset);

    println!("{}kson_rand(1,100);{}", blue, reset);
    println!("{}kson_time();{}", blue, reset);
    println!("{}kson_microtime();{}", blue, reset);
    println!("{}kson_datetime(kson_time());{}", blue, reset);
    println!("{}kson_number_format(123456789);{}", blue, reset);
    println!("{}kson_sleep(3);{}", blue, reset);
    println!("{}kson_tic());{}", blue, reset);
    println!("{}kson_toc());{}", blue, reset);
    println!("{}============================{}", blue, reset);
}
