use std::{
    env, io,
    path::{Path, PathBuf},
    process::Command,
    string::FromUtf8Error,
};

use crate::STARCCM;

#[derive(Debug, thiserror::Error)]
pub enum MacroError {
    #[error("failed to run starcccm+ macro")]
    Command(#[from] io::Error),
    #[error("missing starccm+ podkey")]
    PODKEY(#[from] env::VarError),
    #[error(r#"java macro: "{0}" is missing (you can set the path to the macro with the environment variable: "STARCCM_MACROS")"#)]
    Java(PathBuf),
    #[error("command output message conversion to UTF8 failed")]
    Ouput(#[from] FromUtf8Error),
    #[error("StarCCM+ failed with stdout: {0}")]
    StdOut(String),
    #[error("StarCCM+ failed with stderr: {0}")]
    StdErr(String),
}
type Result<T> = std::result::Result<T, MacroError>;

#[derive(Debug)]
pub struct Macro {
    case_path: PathBuf,
    java_macro: PathBuf,
}
impl Macro {
    pub fn new(case_path: &Path, java_macro: impl Into<PathBuf>) -> Result<Self> {
        let java_macro: PathBuf = java_macro.into();
        if java_macro.is_file() {
            Ok(Self {
                case_path: case_path.to_path_buf(),
                java_macro,
            })
        } else {
            Err(MacroError::Java(java_macro))
        }
    }
    pub fn play(self) -> Result<()> {
        let output = Command::new(&*STARCCM)
            .args([
                "-batch",
                "-power",
                "-podkey",
                &env::var("PODKEY")?,
                "-licpath",
                "1999@flex.cd-adapco.com",
                self.java_macro.to_str().unwrap(),
            ])
            .arg(self.case_path)
            .output()?;
        let stdout = String::from_utf8(output.stdout)?;
        if stdout.contains("Server process ended unexpectedly") {
            return Err(MacroError::StdOut(stdout));
        }
        // let stderr = String::from_utf8(output.stderr)?;
        // if !stderr.is_empty() {
        //     return Err(MacroError::StdErr(stderr));
        // }
        Ok(())
    }
}
