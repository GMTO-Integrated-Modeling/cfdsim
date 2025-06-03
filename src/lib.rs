use std::{env, sync::LazyLock};

use anyhow::Result;
use quick_xml::{Reader, events::Event};

mod checklist;
mod coordinate_systems;
mod expectation;
mod play_macro;
mod test_properties;
mod wind_speed;
pub use checklist::{Check, CheckList};
pub use coordinate_systems::{check_tcs, check_tcs0};
pub use expectation::Expectation;
pub use play_macro::Macro;
pub use test_properties::TestProperty;
pub use wind_speed::WindSpeed;

/// Path to the STARCCM+ binary
pub static STARCCM: LazyLock<String> = LazyLock::new(|| {
    let starcmm = env::var("STARCCM")
        .unwrap_or("/opt/Siemens/17.06.007/STAR-CCM+17.06.007/star/bin/starccm+".to_string());
    println!("Using: {starcmm}");
    starcmm
});
/// Path to the STARCCM+ java macro
pub static STARCCM_MACROS: LazyLock<String> =
    LazyLock::new(|| env::var("STARCCM_MACROS").unwrap_or("/home/ubuntu/Desktop/".to_string()));

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

#[derive(Debug, Default, Clone)]
pub struct Case {
    // name: String,
    zen: u32,
    az: u32,
    config: String,
    // wind_speed: u32,
}
impl Case {
    pub fn new(name: &str) -> Self {
        let mut case_parts = name.split("_");
        let mut zen_az = case_parts.next().unwrap().split("az");
        let zen = zen_az
            .next()
            .unwrap()
            .strip_prefix("zen")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let az = zen_az.last().unwrap().parse::<u32>().unwrap();
        let config = case_parts.next().unwrap().to_string();
        // let wind_speed = case_parts
        //     .next()
        //     .unwrap()
        //     .strip_suffix("ms")
        //     .unwrap()
        //     .parse::<u32>()
        //     .unwrap();
        Self {
            // name: name.to_string(),
            zen,
            az,
            config,
            // wind_speed,
        }
    }
    pub fn parts(&self) -> Vec<String> {
        vec![
            format!("[zen{:02}az{:02}_{}]", self.zen, self.az, self.config),
            format!("[zen{:02}az{:03}_{}]", self.zen, self.az, self.config),
            format!("[zen{:02}az{:02} {}]", self.zen, self.az, self.config),
            format!("[zen{:02}az{:03} {}]", self.zen, self.az, self.config),
            format!("[zen{:02}az{:02}{}]", self.zen, self.az, self.config),
            format!("[zen{:02}az{:03}{}]", self.zen, self.az, self.config),
            format!("[zen{:03}az{:02}_{}]", self.zen, self.az, self.config),
            format!("[zen{:03}az{:03}_{}]", self.zen, self.az, self.config),
            format!("[zen{:03}az{:02} {}]", self.zen, self.az, self.config),
            format!("[zen{:03}az{:03} {}]", self.zen, self.az, self.config),
            format!("[zen{:03}az{:02}{}]", self.zen, self.az, self.config),
            format!("[zen{:03}az{:03}{}]", self.zen, self.az, self.config),
        ]
    }
}
