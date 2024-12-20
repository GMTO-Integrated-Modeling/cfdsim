use std::fmt;

#[derive(Debug, Clone)]
pub struct Expectation<'a>(Vec<&'a str>);
impl<'a> From<&'a str> for Expectation<'a> {
    fn from(value: &'a str) -> Self {
        Self(vec![value])
    }
}
impl<'a> From<Vec<&'a str>> for Expectation<'a> {
    fn from(value: Vec<&'a str>) -> Self {
        Self(value)
    }
}
impl<'a> From<&'a [&'a str]> for Expectation<'a> {
    fn from(value: &'a [&'a str]) -> Self {
        Self(value.to_vec())
    }
}
impl<'a, const N: usize> From<[&'a str; N]> for Expectation<'a> {
    fn from(value: [&'a str; N]) -> Self {
        Self(value.to_vec())
    }
}
impl<'a> PartialEq<str> for Expectation<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0.iter().any(|&v| v == other)
    }
}
impl<'a> fmt::Display for Expectation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() > 1 {
            write!(f, "{:?}", self.0)
        } else {
            write!(f, "{}", self.0[0])
        }
    }
}
