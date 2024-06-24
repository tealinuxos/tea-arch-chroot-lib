use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Keyboard
{
    pub code: String,
    pub name: String,
    pub variant: Vec<Variant>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variant
{
    pub code: String,
    pub name: String
}

impl Keyboard{
    pub fn list() -> HashMap<String, Vec<Self>> {
        let mut keyboard = HashMap::new();
        let mut variants = HashMap::new();

        variants.insert(
            "id".to_string(),
            vec![
                Variant {
                    code: "id".to_string(),
                    name: "Dvorak".to_string(),
                },
                Variant {
                    code: "id".to_string(),
                    name: "Qwerty".to_string(),
                },
            ],
        );

        variants.insert(
            "en".to_string(),
            vec![
                Variant {
                    code: "en".to_string(),
                    name: "Dvorak".to_string(),
                },
                Variant {
                    code: "en".to_string(),
                    name: "Qwerty".to_string(),
                },
            ],
        );

        variants.insert(
            "jp".to_string(),
            vec![
                Variant {
                    code: "jp".to_string(),
                    name: "Dvorak".to_string(),
                },
                Variant {
                    code: "jp".to_string(),
                    name: "Qwerty".to_string(),
                },
            ],
        );

        let value = vec![
            Keyboard {
                code: "id".to_string(),
                name: "Indonesia".to_string(),
                variant: variants.get("id").unwrap().to_vec(),
            },
            Keyboard {
                code: "en".to_string(),
                name: "English".to_string(),
                variant: variants.get("en").unwrap().to_vec(),
            },
            Keyboard {
                code: "jp".to_string(),
                name: "Japan".to_string(),
                variant: variants.get("jp").unwrap().to_vec(),
            },
        ];

        keyboard.insert(String::from("Keyboard"), value);
        keyboard
    }
}