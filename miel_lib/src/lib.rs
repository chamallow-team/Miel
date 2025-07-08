pub mod language;

use crate::language::{LanguageTree, Translation};
use language_tags::LanguageTag;
use std::collections::HashMap;
use std::fmt::Debug;

pub trait TranslationEngine {
    fn create(name: String, version: semver::Version) -> Self;

    // ===========================================
    //  LANGUAGES
    // ===========================================

    fn add_language(&mut self, tag: LanguageTag);

    fn remove_language(&mut self, tag: LanguageTag);

    fn get_language(&self, tag: LanguageTag) -> Option<&LanguageTree>;

    // ===========================================
    //  TRADUCTIONS
    // ===========================================

    fn get_traduction(&self, tag: LanguageTag, key: String) -> Option<&Translation>;

    fn set_traduction(&mut self, tag: LanguageTag, key: String, value: Translation);

    fn remove_traduction(&mut self, tag: LanguageTag, key: String);
}

#[derive(Debug, Clone)]
struct Translations {
    pub name: String,
    pub version: semver::Version,
    pub hash: Option<String>,

    pub languages: HashMap<LanguageTag, language::LanguageTree>,
}

impl TranslationEngine for Translations {
    fn create(name: String, version: semver::Version) -> Self {
        Self {
            name,
            version,
            ..Default::default()
        }
    }

    // ===========================================
    //  LANGUAGES
    // ===========================================

    fn add_language(&mut self, tag: LanguageTag) {
        if !self.languages.contains_key(&tag) {
            self.languages.insert(tag, LanguageTree::new());
        }
    }

    fn remove_language(&mut self, tag: LanguageTag) {
        self.languages.remove(&tag);
    }

    fn get_language(&self, tag: LanguageTag) -> Option<&LanguageTree> {
        self.languages.get(&tag)
    }

    // ===========================================
    //  TRADUCTIONS
    // ===========================================

    fn get_traduction(&self, tag: LanguageTag, key: String) -> Option<&Translation> {
        if !self.languages.contains_key(&tag) {
            return None;
        }
        let language = self.languages.get(&tag).unwrap();

        language.get_traduction(key)
    }

    fn set_traduction(&mut self, tag: LanguageTag, key: String, value: Translation) {
        if !self.languages.contains_key(&tag) {
            self.languages.insert(tag.clone(), LanguageTree::new());
        }
        let language = self.languages.get_mut(&tag).unwrap();
        language.set_traduction(key, value);
    }

    fn remove_traduction(&mut self, tag: LanguageTag, key: String) {
        if !self.languages.contains_key(&tag) {
            return;
        }
        let language = self.languages.get_mut(&tag).unwrap();
        language.remove_traduction(key);
    }
}

impl Default for Translations {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: semver::Version::new(0, 0, 0),
            hash: None,
            languages: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::language::Translation;
    use crate::{TranslationEngine, Translations};
    use std::str::FromStr;

    #[test]
    fn create_translation() {
        let _ = Translations::create("test".to_string(), semver::Version::new(0, 0, 1));
    }

    #[test]
    fn add_languages() {
        let mut translation =
            Translations::create("test".to_string(), semver::Version::new(0, 0, 1));
        translation.add_language(language_tags::LanguageTag::from_str("fr").unwrap());
        translation.add_language(language_tags::LanguageTag::from_str("en-UK").unwrap());
    }

    #[test]
    fn get_language() {
        let mut translation =
            Translations::create("test".to_string(), semver::Version::new(0, 0, 1));

        let fr = language_tags::LanguageTag::from_str("fr").unwrap();

        translation.add_language(fr.clone());

        let _ = translation.get_language(fr);
    }

    #[test]
    fn get_and_set_traductions() {
        let mut translation =
            Translations::create("test".to_string(), semver::Version::new(0, 0, 1));

        let fr = language_tags::LanguageTag::from_str("fr").unwrap();
        translation.add_language(fr.clone());

        translation.set_traduction(
            fr.clone(),
            "home.title".to_string(),
            Translation::new("Ceci est un titre".to_string()),
        );

        let _ = translation.get_traduction(fr, "home.title".to_string());
    }
}
