use std::collections::HashMap;
use std::fs;

use jomini::{JominiDeserialize, TextDeserializer};

use ck3_community_title_project::config;
use ck3_community_title_project::localization;
use ck3_community_title_project::localization::Repository;
use ck3_community_title_project::yaml_loc;

fn main() {
    let cfg = config::Config::new(config::GameConfig::new(""), "doc", config::ModConfig::new("out"));
    let all_loc_lang = localization::Language::all();

    let mut locs: Vec<localization::Localization> = Vec::new();

    let f = fs::File::open(cfg.source_path.join("language.yml")).expect("TODO: panic message");
    let languages: Vec<localization::culture::Language> = serde_yaml::from_reader(f).expect("TODO: panic message");
    locs.push(localization::Localization::new_from(
        "replace/culture/traditions",
        &all_loc_lang,
        "cultural_languages",
        languages,
    ));

    let heritage = vec![localization::culture::Heritage::new("arabic", "ʿArab", "ʿArab", "")];
    locs.push(localization::Localization::new_from(
        "replace/culture/traditions",
        &all_loc_lang,
        "cultural_heritages",
        heritage,
    ));

    let cultures = vec![localization::culture::Culture::new(
        "bedouin", "Badawī", "Badawī", "al-Badū", "",
    )];
    locs.push(localization::Localization::new_from(
        "replace/culture",
        &all_loc_lang,
        "cultures",
        cultures,
    ));

    locs.push(localization::Localization::new(
        "",
        &all_loc_lang,
        "test",
        &[
            localization::Entry::new(
                "sdfs",
                0,
                HashMap::from([
                    (localization::Language::English, "Vive".to_string()),
                    (localization::Language::French, "Viva".to_string()),
                ]),
                "test",
            ),
            localization::Entry::new("", 0, HashMap::new(), "only comment"),
            localization::Entry::new(
                "a3s",
                2,
                HashMap::from([(localization::Language::English, "no comment".to_string())]),
                "",
            ),
        ],
    ));
    let tera = tera::Tera::new("templates/**").unwrap();
    let repo = yaml_loc::Repository::new(cfg.r#mod.localization_path(), &tera);
    locs.iter().for_each(|loc| repo.save_localization(loc).unwrap());

    #[derive(JominiDeserialize, PartialEq, Debug)]
    pub struct Model {
        human: bool,
        first: Option<u16>,
        #[jomini(alias = "forth")]
        fourth: u16,
        #[jomini(alias = "core", duplicated)]
        cores: Vec<String>,
        names: Vec<String>,
    }

    let data = br#"
    human = yes
    forth = 10
    core = "HAB"
    names = { "Johan" "Frederick" }
    core = FRA
"#;

    let expected = Model {
        human: true,
        first: None,
        fourth: 10,
        cores: vec!["HAB".to_string(), "FRA".to_string()],
        names: vec!["Johan".to_string(), "Frederick".to_string()],
    };

    let actual: Model = TextDeserializer::from_windows1252_slice(data)
        .expect("err")
        .deserialize()
        .expect("err");
    assert_eq!(actual, expected);

    #[derive(JominiDeserialize, PartialEq, Debug)]
    pub struct Model1 {
        #[jomini(alias = "type")]
        pub kind: String,
        pub color: String,
    }
    let data1 = br#"
language_czech_slovak = {
	type = language
	is_shown = {
		language_is_shown_trigger = {
			LANGUAGE = language_czech_slovak
		}
	}
	ai_will_do = {
		value = 10
		if = {
			limit = { has_cultural_pillar = language_czech_slovak }
			multiply = 10
		}
	}

	color = czech
}

language_tocharian = {
	type = language
	is_shown = {
		language_is_shown_trigger = {
			LANGUAGE = language_tocharian
		}
	}
	ai_will_do = {
		value = 10
		if = {
			limit = { has_cultural_pillar = language_tocharian }
			multiply = 10
		}
	}

	color = tocharian
}
"#;
    let actual: HashMap<String, Model1> = TextDeserializer::from_utf8_slice(data1)
        .expect("err")
        .deserialize()
        .expect("err");
    println!("{:#?}", actual)
}
