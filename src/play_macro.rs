use std::{
    io,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use crate::{PODKEY, STARCCM};

#[derive(Debug)]
pub struct Macro<'a> {
    case_path: PathBuf,
    java_macro: &'a str,
}
impl<'a> Macro<'a> {
    pub fn new(case_path: &Path, java_macro: &'a str) -> Self {
        Self {
            case_path: case_path.to_path_buf(),
            java_macro,
        }
    }
    pub fn play(self) -> io::Result<Output> {
        Command::new(STARCCM)
            .args([
                "-batch",
                "-power",
                "-podkey",
                PODKEY,
                "-licpath",
                "1999@flex.cd-adapco.com",
                self.java_macro,
            ])
            .arg(self.case_path)
            .output()
    }
}
