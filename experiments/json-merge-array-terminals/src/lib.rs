use serde_json::Value;

pub fn merge(original: &mut Value, new: Value) {
    match (original, new) {
        (Value::Array(original), Value::Array(new)) => {
            original.extend_from_slice(&new);
        }
        (Value::Object(original), Value::Object(new)) => {
            for (k, v) in new {
                if v.is_null() {
                    original.remove(&k);
                } else {
                    merge(original.entry(k).or_insert(Value::Null), v);
                }
            }
        }
        // Otherwise:
        (original, new) => *original = new,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::merge;

    #[test]
    fn it_works() {
        let mut a = json!({
            "a": 1,
            "b": 2,
            "obj": {
                "o-a": 1
            },
            "arr": ["foo", "bar"]
        });

        let b = json!({
            "a": "one",
            "c": 3,
            "obj" : {
                "o-b": 2
            },
            "arr": ["baz"]
        });

        merge(&mut a, b);
        assert_eq!(
            a,
            json!({
                "a": "one",
                "b": 2,
                "c": 3,
                "obj": {
                    "o-a": 1,
                    "o-b": 2,
                },
                "arr": ["foo", "bar", "baz"]
            })
        );
    }
}
