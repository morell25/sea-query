use serde_json::Number;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum Json {
    #[default]
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

impl std::fmt::Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::Value::from(self.clone()))
    }
}

macro_rules! json_from_int {
    ($($t:ty),* $(,)?) => {
        $(
        impl From<$t> for Json {
            fn from(value: $t) -> Self {
                Json::Number(value.into())
            }
        }
        )*
    };
}
json_from_int!(i8, i16, i32, i64, u8, u16, u32, u64);

impl From<f32> for Json {
    fn from(value: f32) -> Self {
        Number::from_f64(value.into()).map_or(Json::Null, Json::Number)
    }
}

impl From<f64> for Json {
    fn from(value: f64) -> Self {
        Number::from_f64(value).map_or(Json::Null, Json::Number)
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
