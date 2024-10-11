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
        temp.as_str().unwrap()
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
        serde_json::from_value::<$t>(temp.clone()).unwrap()
    }};
}
