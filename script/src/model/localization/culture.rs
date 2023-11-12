use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Heritage {
    key: String,
    name: String,
    collective_noun: String,
    comment: String,
}

impl Heritage {
    pub fn new(key: &str, name: &str, collective_noun: &str, comment: &str) -> Self {
        Self {
            key: key.to_string(),
            name: name.to_string(),
            collective_noun: collective_noun.to_string(),
            comment: comment.to_string(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<super::Entry>> for Heritage {
    fn into(self) -> Vec<super::Entry> {
        [
            (format!("heritage_{}_name", &self.key).as_str(), self.name),
            (
                format!("heritage_{}_collective_noun", &self.key).as_str(),
                self.collective_noun,
            ),
        ]
        .iter()
        .map(|a| {
            super::Entry::new(
                a.0,
                0,
                HashMap::from([(super::Language::English, a.1.clone())]),
                &self.comment,
            )
        })
        .collect()
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Language {
    #[serde(alias = "language")]
    key: String,
    name: String,
    #[serde(default)]
    comment: String,
}

impl Language {
    pub fn new(key: &str, name: &str, comment: &str) -> Self {
        Self {
            key: key.to_string(),
            name: name.to_string(),
            comment: comment.to_string(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<super::Entry>> for Language {
    fn into(self) -> Vec<super::Entry> {
        vec![super::Entry::new(
            format!("language_{}_name", &self.key).as_str(),
            0,
            HashMap::from([(super::Language::English, self.name)]),
            &self.comment,
        )]
    }
}

pub struct Culture {
    key: String,
    name: String,
    adjective: String,
    collective_noun: String,
    comment: String,
}

impl Culture {
    pub fn new(key: &str, name: &str, adjective: &str, collective_noun: &str, comment: &str) -> Self {
        Self {
            key: key.to_string(),
            name: name.to_string(),
            adjective: adjective.to_string(),
            collective_noun: collective_noun.to_string(),
            comment: comment.to_string(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<super::Entry>> for Culture {
    fn into(self) -> Vec<super::Entry> {
        [
            (self.key.as_str(), self.name),
            (format!("{}_prefix", &self.key).as_str(), self.adjective),
            (format!("{}_collective_noun", &self.key).as_str(), self.collective_noun),
        ]
        .iter()
        .map(|a| {
            super::Entry::new(
                a.0,
                0,
                HashMap::from([(super::Language::English, a.1.clone())]),
                &self.comment,
            )
        })
        .collect()
    }
}
