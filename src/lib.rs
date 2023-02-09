//! A set of functions that simplifies comparison between two JSON objects. 
use serde_json::Value;

/// A compare function between two JSON Values. Compare returns a boolean true 
/// or false if the Valus are equal. Takes the `exp` argument as the base of 
/// the comparison.
/// 
/// This function compares to see that all values in the `exp` argument are also 
/// present in the `got` argument. It will still return true if `got` has extra
/// object elements not present in `exp`
/// 
/// ```rust
/// 
/// use serde_json::json;
/// use cmp_json::cmp_expected;
/// 
/// let exp = json!{{
///     "some": "value"
/// }};
/// let got_extra = json!{{
///     "some": "value",
///     "another": "element",
/// }};
/// let got_different = json!{{
///     "some": 2
/// }};
/// 
/// assert!(cmp_expected(&got_extra, &exp));
/// assert!(cmp_expected(&got_different, &exp) == false);
/// ```
/// 
pub fn cmp_expected(got: &Value, exp: &Value) -> bool {
    match exp {
        Value::Array(e_arr) => {
            match got.as_array() {
                Some(g_arr) => {
                    if e_arr.len() != g_arr.len() {
                        return false
                    }
                    e_arr.iter().zip(
                        g_arr.iter()
                    ).all(|(e, g)| cmp_expected(g, e))
                }
                None => false
            } 
        }
        Value::Object(e_obj) => {
            match got.as_object() {
                Some(g_obj) => {
                    // We only iterate through expected; if there are values in got that do
                    // not match expected, that is fine
                    e_obj.iter().all(
                        |(k, e_val)| 
                        match g_obj.get(k) {
                            Some(g_val) => cmp_expected(g_val, e_val),
                            None => false
                        }
                    )
                }
                None => false
            }
        }
        _ => got == exp
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value::Null};

    #[test]
    fn null_equal() {
        let got = json!(Null);
        let exp = json!(Null);
        assert_eq!(
            cmp_expected(&got, &exp), 
            true
        );
    }

    #[test]
    fn null_unequal() {
        let got = json!("Null");
        let exp = json!(Null);
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }

    #[test]
    fn string_equal() {
        let got = json!("something");
        let exp = json!("something");
        assert_eq!(
            cmp_expected(&got, &exp), 
            true
        );
    }

    #[test]
    fn string_unequal() {
        let got = json!("something");
        let exp = json!("something else");
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }

    #[test]
    fn number_equal() {
        let got = json!(-12.3);
        let exp = json!(-12.3);
        assert_eq!(
            cmp_expected(&got, &exp), 
            true
        );
    }

    #[test]
    fn number_unequal() {
        let got = json!(-12.5);
        let exp = json!(6);
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }

    #[test]
    fn bool_equal() {
        let got = json!(true);
        let exp = json!(true);
        assert_eq!(
            cmp_expected(&got, &exp), 
            true
        );
    }

    #[test]
    fn bool_unequal() {
        let got = json!(true);
        let exp = json!(false);
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }

    #[test]
    fn vector_equal() {
        let got = json!{["string", 1234, false, [1, 2, 3]]};
        let exp = json!{["string", 1234, false, [1, 2, 3]]};
        assert_eq!(
            cmp_expected(&got, &exp), 
            true
        );
    }

    #[test]
    fn vector_unequal_values() {
        let got = json!{["string", 1234, "false"]};
        let exp = json!{["string", 1234, false]};
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }

    #[test]
    fn vector_unequal_lengths_exp_longer() {
        let got = json!{["string", 1234, false, "extra"]};
        let exp = json!{["string", 1234, false]};
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }

    #[test]
    fn vector_unequal_lengths_got_longer() {
        let got = json!{["string", 1234, false]};
        let exp = json!{["string", 1234, false, "extra"]};
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }

    #[test]
    fn vector_unequal_nested() {
        let got = json!{["string", 1234, false, [1, 2, 3, "wrong"]]};
        let exp = json!{["string", 1234, false, [1, 2, 3]]};
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }

    #[test]
    fn object_equal() {
        let got = json!{{
            "foo": "bar",
            "baz": [
                {"first": true},
                {"second": 2, "third": Null},
            ],
        }};
        let exp = json!{{
            "foo": "bar",
            "baz": [
                {"first": true},
                {"second": 2, "third": Null},
            ],
        }};
        assert_eq!(
            cmp_expected(&got, &exp), 
            true
        );
    }

    #[test]
    fn object_equal_extra_got() {
        let got = json!{{
            "foo": "bar",
            "baz": [
                {"first": true},
                {"second": 2, "third": Null},
            ],
            "another": "field",
        }};
        let exp = json!{{
            "foo": "bar",
            "baz": [
                {"first": true},
                {"second": 2, "third": Null},
            ],
        }};
        assert_eq!(
            cmp_expected(&got, &exp), 
            true
        );
    }

    #[test]
    fn object_unequal_extra_exp() {
        let got = json!{{
            "foo": "bar",
            "baz": [
                {"first": true},
                {"second": 2, "third": Null},
            ],
        }};
        let exp = json!{{
            "foo": "bar",
            "baz": [
                {"first": true},
                {"second": 2, "third": Null},
            ],
            "another": "field",
        }};
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }

    #[test]
    fn object_unequal_values() {
        let got = json!{{
            "foo": "bar!!",
            "baz": [
                {"first": true},
                {"second": 2, "third": Null},
            ],
        }};
        let exp = json!{{
            "foo": "bar",
            "baz": [
                {"first": true},
                {"second": 2, "third": Null},
            ],
        }};
        assert_eq!(
            cmp_expected(&got, &exp), 
            false
        );
    }
}
