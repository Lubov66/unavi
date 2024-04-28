use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Instance {
    pub world: World,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct World {
    pub did: String,
    pub record: String,
}

#[cfg(test)]
mod tests {
    use jsonschema::JSONSchema;

    use super::*;

    const SCHEMA: &[u8] =
        include_bytes!("../../../../wired-protocol/social/dwn/schemas/instance.json");

    #[test]
    fn test_schema() {
        let instance = Instance {
            world: World {
                did: "did:example:123".to_string(),
                record: "abcde".to_string(),
            },
        };

        let serialized = serde_json::to_vec(&instance).unwrap();
        let deserialized = serde_json::from_slice(&serialized).unwrap();

        let schema = serde_json::from_slice(SCHEMA).unwrap();
        let schema = JSONSchema::compile(&schema).unwrap();

        if schema.validate(&deserialized).is_err() {
            panic!("Failed to validate");
        };
    }
}
