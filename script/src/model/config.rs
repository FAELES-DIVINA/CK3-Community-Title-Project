use std::path::PathBuf;

pub struct GameConfig {
    game_root_path: Box<PathBuf>,
}

pub struct ModConfig {
    mod_root_path: Box<PathBuf>,
}

pub struct Config {
    pub game: GameConfig,
    pub source_path: Box<PathBuf>,
    pub r#mod: ModConfig,
}

impl GameConfig {
    pub fn new(game_root_path: &str) -> Self {
        Self {
            game_root_path: Box::from(PathBuf::from(game_root_path)),
        }
    }
    pub fn localization_path(&self) -> PathBuf {
        self.game_root_path.join("game").join("localization")
    }
}

impl ModConfig {
    pub fn new(mod_root_path: &str) -> Self {
        Self {
            mod_root_path: Box::from(PathBuf::from(mod_root_path)),
        }
    }
    pub fn localization_path(&self) -> PathBuf {
        self.mod_root_path.join("localization")
    }
}

impl Config {
    pub fn new(game: GameConfig, source_path: &str, r#mod: ModConfig) -> Self {
        Self {
            game,
            source_path: Box::from(PathBuf::from(source_path)),
            r#mod,
        }
    }
}
