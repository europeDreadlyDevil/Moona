use std::env;
use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct CLI {
    path: Option<PathBuf>,
    ///Run current rule
    #[arg(short, long)]
    rule: Option<String>,
}

impl CLI {
    pub fn get_path(&self) -> PathBuf {
        if let Some(path) = &self.path {
            path.clone()
        } else {
            let wd = WalkDir::new(env::current_dir().unwrap());
            for dir in wd.into_iter().flatten() {
                if dir.file_name().to_str().unwrap().to_lowercase() == *"make.toml" {
                    return dir.path().to_path_buf()
                }
            }
            panic!("Make.toml not found")
        }
    }

    pub fn get_rule(&self) -> Option<String> {
        self.rule.clone()
    }
}