use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use anyhow::anyhow;

use crate::common::Result;

pub mod culture;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Language {
    English,
    French,
    German,
    Korean,
    Russian,
    SimpChinese,
    Spanish,
}

impl Language {
    pub fn all() -> Vec<Self> {
        vec![
            Language::English,
            Language::French,
            Language::German,
            Language::Korean,
            Language::Russian,
            Language::SimpChinese,
            Language::Spanish,
        ]
    }
    pub(crate) fn from_header(s: &str) -> Result<Self> {
        match s {
            "l_english" => Ok(Language::English),
            "l_french" => Ok(Language::French),
            "l_german" => Ok(Language::German),
            "l_korean" => Ok(Language::Korean),
            "l_russian" => Ok(Language::Russian),
            "l_simp_chinese" => Ok(Language::SimpChinese),
            "l_spanish" => Ok(Language::Spanish),
            _ => Err(anyhow!("Unknown language: {}", s)),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::English => write!(f, "english"),
            Language::French => write!(f, "french"),
            Language::German => write!(f, "german"),
            Language::Korean => write!(f, "korean"),
            Language::Russian => write!(f, "russian"),
            Language::SimpChinese => write!(f, "simp_chinese"),
            Language::Spanish => write!(f, "spanish"),
        }
    }
}

#[derive(Clone)]
pub struct Entry {
    key: String,
    version: u8,
    content: HashMap<Language, String>,
    comment: String,
}

impl Entry {
    pub fn new(key: &str, version: u8, content: HashMap<Language, String>, comment: &str) -> Self {
        Self {
            key: key.to_string(),
            version,
            content,
            comment: comment.to_string(),
        }
    }
    pub(crate) fn key(&self) -> &str {
        &self.key
    }
    pub(crate) fn content_of(&self, lang: &Language) -> &str {
        return self
            .content
            .get(lang)
            .or(self.content.get(&Language::English))
            .and_then(|s| Option::from(s.as_str()))
            .unwrap_or("");
    }
    pub(crate) fn version(&self) -> u8 {
        self.version
    }
    pub(crate) fn comment(&self) -> &str {
        &self.comment
    }
}

pub struct Localization {
    path: String,
    lang: Vec<Language>,
    name: String,
    entries: Vec<Entry>,
}

impl Localization {
    pub fn new(path: &str, lang: &[Language], name: &str, entries: &[Entry]) -> Self {
        Self {
            path: path.to_string(),
            lang: Vec::from(lang),
            name: name.to_string(),
            entries: Vec::from(entries),
        }
    }
    pub fn new_from<T: Into<Vec<Entry>>>(path: &str, lang: &[Language], name: &str, entries: Vec<T>) -> Self {
        Self {
            path: path.to_string(),
            lang: Vec::from(lang),
            name: name.to_string(),
            entries: to_entries(entries),
        }
    }
    pub(crate) fn path(&self) -> &str {
        &self.path
    }
    pub(crate) fn lang(&self) -> &Vec<Language> {
        &self.lang
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}

pub trait Repository {
    fn load_localization(&self, path: &str, lang: &[Language], name: &str) -> Result<Localization>;
    fn save_localization(&self, loc: &Localization) -> Result<()>;
}

fn to_entries<T: Into<Vec<Entry>>>(values: Vec<T>) -> Vec<Entry> {
    values.into_iter().flat_map(|h| -> Vec<Entry> { h.into() }).collect()
}
