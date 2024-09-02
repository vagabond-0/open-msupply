use std::collections::HashMap ;
use serde_yaml::Value;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
// Relative to server/Cargo.toml
// later this will be client in dev mode, or build in production mode
#[folder = "../../client/packages/host/dist/locales"]
pub struct EmbeddedLocalisations;



// struct to manage translations
pub struct Localisations {
    pub translations: HashMap<String, HashMap<String, HashMap<String, String>>>,
}

pub struct TranslationStrings {
    pub translations: HashMap<String, Value>,
}

impl Localisations {

    // Creates a new Localisations struct
    pub fn new() -> Self {
        Localisations {
            translations: HashMap::new(),
        }
    }

    // Load translations from embedded files
    pub fn load_translations(&mut self) -> Result<(), std::io::Error> {
        // add read all namespace file names within locales
        for file in EmbeddedLocalisations::iter() {
                let file_namespace = file.split('/').nth(1).unwrap_or_default().to_string();
                let language = file.split('/').nth(0).unwrap_or_default().to_string();
                if let Some(content) = EmbeddedLocalisations::get(&file) {
                    let json_data = content.data;
                    let translations: HashMap<String, String> = serde_json::from_slice(&json_data).unwrap();
                    self.translations
                    .entry(language)
                    .or_insert_with(HashMap::new)
                    .insert(file_namespace, translations);
            }
        }
        Ok(())
    }

    // Get a translation for a given key and language
    // next need to add fallback and namespace to get Translation function
    pub fn get_translation(&self, args: &HashMap<String, serde_json::Value> ) -> String {
        let key = args.get("key").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
        let lang = args.get("lang").and_then(serde_json::Value::as_str).unwrap_or("en").to_string();
        let namespace = args.get("namespace").and_then(serde_json::Value::as_str).unwrap_or("reports.json").to_string();
        let fallback = args.get("fallback").and_then(serde_json::Value::as_str).unwrap_or("").to_string();

        self.translations
            .get(&lang)
            .and_then(|map| map.get(&namespace))
            .and_then(|map| map.get(&key))
            .cloned()
            .unwrap_or_else(|| fallback)
    }
}

#[cfg(test)]
mod test {

use std::collections::HashMap;

use super::Localisations;


    #[test]
    fn test_translations() {
        let mut localisations = Localisations::new();
        // test loading localisations
        // note these translations might change if translations change in the front end. In this case, these will need to be updated.
        let _ = localisations.load_translations();

        let mut args = HashMap::new();
        args.insert("key".to_string(), serde_json::Value::String("button.close".to_owned()));
        args.insert("lang".to_string(), serde_json::Value::String("fr".to_owned()));
        args.insert("namespace".to_string(), serde_json::Value::String("common.json".to_owned()));
        args.insert("fallback".to_string(), serde_json::Value::String("fallback".to_owned()));
        // test correct translation
        let translated_value = localisations.get_translation(&args);      
        assert_eq!("Fermer", translated_value);
        // test wrong key fallback
        args.insert("key".to_string(),serde_json::Value::String("button.close-non-existent-key".to_owned()));
        args.insert("fallback".to_string(), serde_json::Value::String("fallback wrong key".to_owned()));
        let translated_value = localisations.get_translation(&args);      
        assert_eq!("fallback wrong key", translated_value);        
        // // test wrong language dir
        args.insert("key".to_string(), serde_json::Value::String("button.close".to_owned()));
        args.insert("lang".to_string(), serde_json::Value::String("fr-non-existent-lang".to_owned()));
        args.insert("fallback".to_string(), serde_json::Value::String("fallback wrong lang dir".to_owned()));
        let translated_value = localisations.get_translation(&args);      
        assert_eq!("fallback wrong lang dir", translated_value);
        // test wrong namespace
        args.insert("lang".to_string(), serde_json::Value::String("fr".to_owned()));
        args.insert("namespace".to_string(), serde_json::Value::String("common.json-non-existent-file".to_owned()));
        args.insert("fallback".to_string(), serde_json::Value::String("fallback wrong namespace".to_owned()));
        let translated_value = localisations.get_translation(&args);      
        assert_eq!("fallback wrong namespace", translated_value);
        // test other lang file
        args.insert("lang".to_string(), serde_json::Value::String("es".to_owned()));
        args.insert("namespace".to_string(), serde_json::Value::String("common.json".to_owned()));
        args.insert("fallback".to_string(), serde_json::Value::String("fallback".to_owned()));
        let translated_value = localisations.get_translation(&args);      
        assert_eq!("Cerrar", translated_value);
    }
}
