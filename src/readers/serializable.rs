use serde_json::Value;
use serde_yaml::Value as YamlValue;
use serde::{Deserialize, Serialize};

pub trait MySerializable {
    fn to_json(&self) -> Value;
    fn to_yaml(&self) -> YamlValue;
}

impl<T> MySerializable for T where T: Serialize + Deserialize<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(self).expect("JSON serialization failed")
    }

    fn to_yaml(&self) -> YamlValue {
        serde_yaml::to_value(self).expect("YAML serialization failed")
    }
}