use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::Context;
use tera::Tera;

use crate::common::Result;
use crate::model::localization;

#[derive(serde::Serialize)]
struct Line {
    key: String,
    version: u8,
    content: String,
    comment: String,
}

impl Line {
    pub fn new(key: &str, version: u8, content: &str, comment: &str) -> Self {
        Self {
            key: key.to_string(),
            version,
            content: content.to_string(),
            comment: comment.to_string(),
        }
    }
}

// YMLFile is yaml localization file used in CK III
struct YMLFile {
    name: String,
    language: localization::Language,
    lines: Vec<Line>,
}

impl YMLFile {
    fn new(name: &str, language: &localization::Language, num_lines: usize) -> Self {
        Self {
            name: name.to_string(),
            language: language.clone(),
            lines: Vec::with_capacity(num_lines),
        }
    }
    // file_name of the file
    fn file_name(&self) -> String {
        format!("{}_l_{}.yml", self.name, self.language)
    }
}

pub struct Repository<'a> {
    localization_root: PathBuf,
    tmpl: &'a Tera,
}

impl<'a> Repository<'a> {
    fn load_file(&self, path: PathBuf, _name: &str) -> Result<YMLFile> {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file = fs::File::open(&path)?;
        let lines_amount = BufReader::new(&file).lines().count() - 1;
        let file = fs::File::open(&path)?;
        let mut lines = BufReader::new(&file).lines();

        let language = localization::Language::from_header(lines.next().unwrap()?.as_str())?;
        let f = YMLFile::new(file_name.split_once('_').unwrap().0, &language, lines_amount);
        for _line in lines {
            _ = f.name.lines();
        }
        Ok(f)
    }

    fn save_file(&self, path: &str, file: &YMLFile) -> Result<()> {
        let path = self
            .localization_root
            .join(file.language.to_string())
            .join(path)
            .join(file.file_name());
        let path = path.as_path();

        let mut context = tera::Context::new();
        context.insert("language", &file.language.to_string());
        context.insert("lines", file.lines.as_slice());
        let content = self.tmpl.render("loc.yml", &context)?;

        let prefix = path.parent();
        match prefix {
            None => {}
            Some(prefix) => fs::create_dir_all(prefix)?,
        };
        let bom: [u8; 3] = [0xEF, 0xBB, 0xBF];
        let content = [bom.as_slice(), content.as_bytes()].concat();
        fs::write(path, content).with_context(|| format!("write file {} failed.", path.display()))?;
        Ok(())
    }
    pub fn new(localization_root: PathBuf, tmpl: &'a Tera) -> Repository<'a> {
        Self {
            localization_root,
            tmpl,
        }
    }
}

impl<'a> localization::Repository for Repository<'a> {
    fn load_localization(
        &self,
        _path: &str,
        _lang: &[localization::Language],
        _name: &str,
    ) -> Result<localization::Localization> {
        todo!()
    }

    fn save_localization(&self, loc: &localization::Localization) -> Result<()> {
        let mut files: HashMap<&localization::Language, YMLFile> = loc
            .lang()
            .iter()
            .map(|lang| (lang, YMLFile::new(loc.name(), lang, loc.entries().len())))
            .collect();

        loc.entries().iter().for_each(|entry| {
            files.iter_mut().for_each(|(lang, file)| {
                file.lines.push(Line::new(
                    entry.key(),
                    entry.version(),
                    entry.content_of(lang),
                    entry.comment(),
                ))
            })
        });
        for file in files {
            self.save_file(loc.path(), &file.1)?;
        }
        Ok(())
    }
}
