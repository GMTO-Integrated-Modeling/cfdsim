use crate::CfdCheckListError;

#[derive(Debug, Clone)]
pub enum WindSpeed {
    Two,
    Seven,
    Twelve,
    Seventeen,
}
impl WindSpeed {
    pub fn new(case: &str) -> Result<Self, CfdCheckListError> {
        match case
            .split("_")
            .last()
            .unwrap()
            .strip_suffix("ms")
            .unwrap()
            .parse::<u32>()
            .unwrap()
        {
            2 => Ok(WindSpeed::Two),
            7 => Ok(WindSpeed::Seven),
            12 => Ok(WindSpeed::Twelve),
            17 => Ok(WindSpeed::Seventeen),
            w => Err(CfdCheckListError::WindSpeed(w)),
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
            WindSpeed::Seven => vec!["4*.954", "4*0.954"],
            WindSpeed::Twelve => vec!["7*.978", "7*0.978"],
            WindSpeed::Seventeen => vec!["10*.978", "10*0.978"],
        }
    }
}
