use super::*;

#[derive(Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl From<serde_json::Value> for Json {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Json::Null,
            serde_json::Value::Bool(value) => Json::Bool(value),
            serde_json::Value::Number(value) => Json::Number(value),
            serde_json::Value::String(value) => Json::String(value),
            serde_json::Value::Array(value) => {
                Json::Array(value.into_iter().map(Json::from).collect())
            }
            serde_json::Value::Object(value) => Json::Object(
                value
                    .into_iter()
                    .map(|(x, v)| {
                        let v1 = Json::from(v);
                        (x, v1)
                    })
                    .collect(),
            ),
        }
    }
}

impl From<Json> for serde_json::Value {
    fn from(value: Json) -> Self {
        match value {
            Json::Null => serde_json::Value::Null,
            Json::Bool(value) => serde_json::Value::Bool(value),
            Json::Number(value) => serde_json::Value::Number(value),
            Json::String(value) => serde_json::Value::String(value),
            Json::Array(value) => {
                serde_json::Value::Array(value.into_iter().map(serde_json::Value::from).collect())
            }
            Json::Object(value) => serde_json::Value::Object(
                value
                    .into_iter()
                    .map(|(x, v)| {
                        let v1 = serde_json::Value::from(v);
                        (x, v1)
                    })
                    .collect(),
            ),
        }
    }
}

#[test]
fn jot() {
    assert!(std::mem::size_of::<Json>() > 0);
}

#[test]
fn jot2() {
    let original = serde_json::json!({ "name": "Alice", "age": 30, "active": true, "items": [1, 2, 3], "nothing": null });
    let json = Json::from(original.clone());
    let converted = serde_json::Value::from(json);
    assert_eq!(converted, original);
}
