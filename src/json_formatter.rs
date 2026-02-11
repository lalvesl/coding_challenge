use serde_json::Value;

const MAX_WIDTH: usize = 80;
const INDENT_WIDTH: usize = 2;

pub fn format_recursive(value: &Value, indent_level: usize) -> String {
    let compact = serde_json::to_string(value).unwrap_or_default();
    let current_indent_str = " ".repeat(indent_level * INDENT_WIDTH);

    if current_indent_str.len() + compact.len() <= MAX_WIDTH {
        return compact;
    }

    match value {
        Value::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            let mut s = String::new();
            s.push('[');
            s.push('\n');
            for (i, item) in arr.iter().enumerate() {
                let item_str = format_recursive(item, indent_level + 1);
                s.push_str(&" ".repeat((indent_level + 1) * INDENT_WIDTH));
                s.push_str(&item_str);

                if i < arr.len() - 1 {
                    s.push(',');
                }
                s.push('\n');
            }
            s.push_str(&current_indent_str);
            s.push(']');
            s
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                return "{}".to_string();
            }
            let mut s = String::new();
            s.push('{');
            s.push('\n');
            for (i, (k, v)) in obj.iter().enumerate() {
                let key_str = serde_json::to_string(k).unwrap_or_else(|_| format!("\"{}\"", k));
                let val_str = format_recursive(v, indent_level + 1);

                s.push_str(&" ".repeat((indent_level + 1) * INDENT_WIDTH));
                s.push_str(&key_str);
                s.push_str(": ");
                s.push_str(&val_str);

                if i < obj.len() - 1 {
                    s.push(',');
                }
                s.push('\n');
            }
            s.push_str(&current_indent_str);
            s.push('}');
            s
        }
        _ => compact,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_small_array_inline() {
        let v = json!([1, 2, 3]);
        let s = format_recursive(&v, 0);
        assert_eq!(s, "[1,2,3]");
    }

    #[test]
    fn test_small_object_inline() {
        let v = json!({"a": 1, "b": 2});
        let s = format_recursive(&v, 0);
        // keys are sorted in default serde_json::Value (BTreeMap)
        assert_eq!(s, "{\"a\":1,\"b\":2}");
    }

    #[test]
    fn test_large_array_expanded() {
        // Create an array that definitely exceeds 80 chars
        let mut arr = Vec::new();
        for i in 0..30 {
            arr.push(i);
        }
        let v = json!(arr);
        let s = format_recursive(&v, 0);
        assert!(s.contains("[\n"));
        assert!(s.contains("  0,\n"));
        assert!(s.ends_with("\n]"));
    }

    #[test]
    fn test_nested_mixed() {
        let v = json!([
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"},
            {"id": 3, "name": "Charlie_Long_Name_To_Force_Expansion_Of_Outer_Array"}
        ]);
        let s = format_recursive(&v, 0);
        assert!(s.starts_with("[\n"));
        assert!(s.contains("  {\"id\":1,\"name\":\"Alice\"},"));
    }
}
