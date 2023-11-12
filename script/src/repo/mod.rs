use std::path::PathBuf;

pub mod yaml_loc;

pub trait Config {
    fn localization_path(&self) -> PathBuf;
}
