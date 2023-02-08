use serde_json::{json, Value};

pub fn cmp_json_expected(got: &Value, exp: &Value) -> bool {
    match exp {
        Value::Array(e_arr) => {
            match got.as_array() {
                Some(g_arr) => {
                    e_arr.iter().zip(
                        g_arr.iter()
                    ).all(|(e, g)| cmp_json_expected(g, e))
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
                            Some(g_val) => cmp_json_expected(g_val, e_val),
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

    #[test]
    fn string_equal() {
        let got = json!("something");
        let exp = json!("something");
        assert_eq!(
            cmp_json_expected(&got, &exp), 
            true
        );
    }

    #[test]
    fn string_unequal() {
        let got = json!("something");
        let exp = json!("something else");
        assert_eq!(
            cmp_json_expected(&got, &exp), 
            false
        );
    }
}
