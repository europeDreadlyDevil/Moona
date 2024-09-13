use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(about)]
pub struct CLI {
    path: PathBuf
}

impl CLI {
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}