use quick_xml::{Reader, events::Event};

use crate::{CfdCheckListError, Check, Expectation};

pub struct TestProperty<'a> {
    prop_event: Vec<(&'a str, &'a [u8])>,
    sub_event: &'a [u8],
    setting: Expectation<'a>,
}

impl<'a> TestProperty<'a> {
    pub fn new(
        prop_event: Vec<(&'a str, &'a [u8])>,
        sub_event: &'a [u8],
        setting: impl Into<Expectation<'a>>,
    ) -> Self {
        Self {
            prop_event,
            sub_event,
            setting: setting.into(),
        }
    }
    pub fn probe(self, file: &'a str) -> Result<Check<'a>, CfdCheckListError> {
        let prop = self.prop_event[0].0.to_string();
        let setting = self.setting.clone();
        match self.property(file) {
            Ok(val) => {
                if setting == *val.as_str() {
                    Ok(Check::Pass((prop, val).into()))
                } else {
                    Ok(Check::Fail((prop, val, setting).into()))
                }
            }
            Err(e) => Err(e),
        }
    }
    pub fn parts(self, file: &str) -> Result<Vec<String>, CfdCheckListError> {
        self.property(file).map(|p| {
            p.strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .split(",")
                .map(|p| p.split(".").last().unwrap().to_string())
                .collect::<Vec<_>>()
        })
    }
    pub fn check_ducts(self, file: &str) -> Result<Check<'a>, CfdCheckListError> {
        let region = "ducts";
        let parts = self.parts(file)?;
        if parts.len() == 1 && (parts[0] == "duct" || parts[0] == "ducts") {
            Ok(Check::Pass((region, format!("{:?}", parts)).into()))
        } else {
            Ok(Check::Fail((region, "[duct,dutcs]").into()))
        }
    }
    pub fn check_ws(self, file: &str) -> Result<Check<'a>, CfdCheckListError> {
        let region = "ws";
        let parts = self.parts(file)?;
        if parts.len() == 2 && parts[0] == region && parts[1] == "beam" {
            Ok(Check::Pass((region, format!("{:?}", parts)).into()))
        } else {
            Ok(Check::Fail((region, "[ws,beam]").into()))
        }
    }
    pub fn check_instvol(self, file: &str) -> Result<Check<'a>, CfdCheckListError> {
        let region = "instvol";
        let parts = self.parts(file)?;
        if parts.len() == 2 && parts[0] == "instvol" && parts[1] == "GCLEFvol" {
            // println!("PASS: {:<22} = {:?}", region, parts);
            Ok(Check::Pass((region, format!("{:?}", parts)).into()))
        } else {
            Ok(Check::Fail((region, "[instvol,GCLEFvol]").into()))
        }
    }
    pub fn property(self, file: &str) -> Result<String, CfdCheckListError> {
        let mut reader = Reader::from_file(file)?;
        let mut buf = Vec::new();
        for (prop, event) in self.prop_event.into_iter() {
            loop {
                match reader.read_event_into(&mut buf)? {
                    Event::Start(e) => {
                        if e.name().as_ref() == event {
                            if let Ok(Some(attr)) = e.try_get_attribute("PresentationName") {
                                if attr.value.as_ref() == prop.to_string().into_bytes() {
                                    break;
                                }
                            }
                        }
                    }
                    Event::Eof => {
                        return Err(CfdCheckListError::Setting(
                            String::from_utf8(event.to_vec()).unwrap(),
                        ));
                    }
                    _ => (),
                }
            }
        }
        let val = loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) => {
                    if e.name().as_ref() == self.sub_event {
                        if let Event::Text(t) = reader.read_event_into(&mut buf)? {
                            let txt = t.unescape().unwrap().into_owned();
                            break txt;
                        }
                    }
                }
                Event::Eof => {
                    println!(
                        r#"FAIL: missing "{}""#,
                        String::from_utf8_lossy(self.sub_event)
                    );
                    return Err(CfdCheckListError::Setting(String::new()));
                }
                _ => (),
            }
        };
        Ok(val)
    }
}
