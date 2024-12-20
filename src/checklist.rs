use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::Expectation;

#[derive(Debug, Clone)]
pub struct CheckData<'a> {
    prop: String,
    value: String,
    setting: Option<Expectation<'a>>,
}
impl<'a> From<(&'a str, &'a str)> for CheckData<'a> {
    fn from((prop, value): (&'a str, &'a str)) -> Self {
        Self {
            prop: prop.to_string(),
            value: value.to_string(),
            setting: None,
        }
    }
}
impl<'a> From<(&'a str, String)> for CheckData<'a> {
    fn from((prop, value): (&'a str, String)) -> Self {
        Self {
            prop: prop.to_string(),
            value,
            setting: None,
        }
    }
}
impl<'a> From<(String, String)> for CheckData<'a> {
    fn from((prop, value): (String, String)) -> Self {
        Self {
            prop,
            value,
            setting: None,
        }
    }
}
impl<'a> From<(String, String, Expectation<'a>)> for CheckData<'a> {
    fn from((prop, value, setting): (String, String, Expectation<'a>)) -> Self {
        Self {
            prop,
            value,
            setting: Some(setting),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Check<'a> {
    Pass(CheckData<'a>),
    Fail(CheckData<'a>),
}
impl<'a> Display for Check<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Check::Pass(cd) => write!(f, "PASS: {:<22} = {}", cd.prop, cd.value),
            Check::Fail(cd) => {
                if let Some(setting) = cd.setting.as_ref() {
                    write!(
                        f,
                        r#"FAIL: {:<22} ~ {} (expected: "{}")"#,
                        cd.prop, cd.value, setting
                    )
                } else {
                    write!(f, "FAIL: {:<22} ~ {}", cd.prop, cd.value)
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct CheckList<'a>(pub(crate) Vec<Check<'a>>);
impl<'a> CheckList<'a> {
    pub fn pass(&self) -> bool {
        !self.0.iter().any(|check| {
            if let Check::Fail(_) = check {
                true
            } else {
                false
            }
        })
    }
}
impl<'a> Display for CheckList<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for check in self.0.iter() {
            writeln!(f, "{check}")?;
        }
        if self.pass() {
            write!(f, "CHECKS SUCCESSFUL")?;
        } else {
            write!(f, "CHECKS FAILED")?;
        }
        Ok(())
    }
}
impl<'a> Deref for CheckList<'a> {
    type Target = Vec<Check<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> DerefMut for CheckList<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
