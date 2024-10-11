// tests/test_kson.rs
use kson::kson;

#[test]
fn test_kson_macro() {
    kson!(a);
    kson!(a["name"] = "kinggunil");
    kson!(a["age"] = 40);
    kson!(a["phone"]["office"] = "010-28**-3440");
    kson!(a["phone"]["home"] = "031-7**-2440");

    kson!(a["country"][0] = "Korea");
    kson!(a["country"][1] = "Canada");
    kson!(a["like"]["number"] = 777);

    assert_eq!(kson!(a["name"]: &str), "kinggunil");
    assert_eq!(kson!(a["age"]: i32), 40);
    assert_eq!(kson!(a["phone"]["office"]: &str), "010-28**-3440");
    assert_eq!(kson!(a["phone"]["home"]: &str), "031-7**-2440");
    assert_eq!(kson!(a["country"][0]: &str), "Korea");
    assert_eq!(kson!(a["country"][1]: &str), "Canada");
    assert_eq!(kson!(a["like"]["number"]: i32), 777);
}
