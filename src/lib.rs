use anyhow::Result;
use quick_xml::{events::Event, Reader};

mod checklist;
mod coordinate_systems;
mod expectation;
mod test_properties;
mod wind_speed;
pub use checklist::{Check, CheckList};
pub use coordinate_systems::{check_tcs, check_tcs0};
pub use expectation::Expectation;
pub use test_properties::TestProperty;
pub use wind_speed::WindSpeed;

#[derive(Debug, thiserror::Error)]
pub enum CfdCheckListError {
    #[error("wrong CFD setting for ({0})")]
    Setting(String),
    #[error("coordinate system mismatch between {0} and {1}")]
    CoordinateSystem(String, String),
    #[error("wrong parts for {0}: {1:?}")]
    Parts(String, Vec<String>),
    #[error("report do not match case: {0}")]
    Mismatch(String),
    #[error("found windspeed {0}ms, expected 2m/s, 7m/s, 12m/s or 17m/s")]
    WindSpeed(u32),
    #[error("faile to parse the XML CFD report")]
    Xml(#[from] quick_xml::Error),
}

pub struct Tests<'a> {
    file: &'a str,
    properties: Vec<TestProperty<'a>>,
}
impl<'a> Tests<'a> {
    pub fn new(file: &'a str, properties: Vec<TestProperty<'a>>) -> Self {
        Self { file, properties }
    }
    pub fn run(self) -> Result<Vec<Check<'a>>, CfdCheckListError> {
        self.properties
            .into_iter()
            .map(|tp| tp.probe(self.file))
            .collect()
    }
}

impl<'a> TryFrom<Tests<'a>> for CheckList<'a> {
    type Error = CfdCheckListError;

    fn try_from(tests: Tests<'a>) -> std::result::Result<Self, Self::Error> {
        Ok(CheckList(tests.run()?))
    }
}

pub fn match_report_to_case(file: &str, case: &str) -> Result<(), CfdCheckListError> {
    let mut reader = Reader::from_file(file)?;
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"SummaryReport" {
                    if let Ok(Some(attr)) = e.try_get_attribute("Name") {
                        if attr.value.as_ref() == case.to_string().into_bytes() {
                            break Ok(());
                        } else {
                            break Err(CfdCheckListError::Mismatch(case.to_string()));
                        }
                    }
                }
            }
            Event::Eof => {
                println!(r#"FAIL: missing "SummaryReport""#);
                break Err(CfdCheckListError::Mismatch(case.to_string()));
            }
            _ => (),
        }
    }
}
