use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum WindSpeedError {
    #[error("found windspeed {0}ms, expected 2m/s, 7m/s, 12m/s or 17m/s")]
    WindSpeed(u32),
    #[error(r#"failed to split wind case with repect to "_""#)]
    Split,
    #[error(r#"failed to strip "ms" suffix from case name"#)]
    Strip,
    #[error("failed to convert wind speed to numerical value")]
    Parser(#[from] ParseIntError),
}

#[derive(Debug, Clone)]
pub enum WindSpeed {
    Two,
    Seven,
    Twelve,
    Seventeen,
}
impl WindSpeed {
    pub fn new(case: &str) -> Result<Self, WindSpeedError> {
        match case
            .split("_")
            .last()
            .ok_or(WindSpeedError::Split)?
            .strip_suffix("ms")
            .ok_or(WindSpeedError::Strip)?
            .parse::<u32>()?
        {
            2 => Ok(WindSpeed::Two),
            7 => Ok(WindSpeed::Seven),
            12 => Ok(WindSpeed::Twelve),
            17 => Ok(WindSpeed::Seventeen),
            w => Err(WindSpeedError::WindSpeed(w)),
        }
    }
    pub fn duration(&self) -> &str {
        match self {
            Self::Two => "1200.0 s",
            _ => "900.0 s",
        }
    }
    pub fn start(&self) -> &str {
        match self {
            Self::Two => "800.0 s",
            _ => "500.0 s",
        }
    }
    pub fn scene_start(&self) -> &[&str] {
        match self {
            Self::Two => &["300.0 s", "500.0 s", "800.0 s"],
            _ => &["300.0 s", "500.0 s"],
        }
    }
    pub fn u_max(&self) -> Vec<&str> {
        match self {
            WindSpeed::Two => vec!["1.2*.922", "1.2*0.922"],
            WindSpeed::Seven => vec!["4*.954", "4*0.954", "4 *.954", "4 *0.954"],
            WindSpeed::Twelve => vec!["7*.978", "7*0.978"],
            WindSpeed::Seventeen => vec!["10*.978", "10*0.978"],
        }
    }
}
