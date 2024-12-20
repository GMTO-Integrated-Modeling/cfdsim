use crate::{CfdCheckListError, Check, TestProperty};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CoordinateSystem {
    // name: String,
    x_vector: String,
    xy_plane: String,
    basis_0: String,
    basis_1: String,
    basis_2: String,
    origin: String,
}
impl CoordinateSystem {
    pub fn new(file: &str, name: &str) -> Result<Self, CfdCheckListError> {
        let prop = |sub_event: &[u8]| {
            TestProperty::new(
                vec![
                    ("Coordinate Systems", b"commonCoordinateSystemManager"),
                    (name, b"commonCartesianCoordinateSystem"),
                ],
                sub_event,
                "",
            )
            .property(file)
        };
        Ok(Self {
            // name: name.to_string(),
            x_vector: prop(b"XVector")?,
            xy_plane: prop(b"XyPlane")?,
            basis_0: prop(b"Basis0")?,
            basis_1: prop(b"Basis1")?,
            basis_2: prop(b"Basis2")?,
            origin: prop(b"Origin")?,
        })
    }
}

pub fn check_tcs<'a>(file: &'a str, case: &'a str) -> Result<Check<'a>, CfdCheckListError> {
    let mut zen_az = case.split("_").next().unwrap().split("az");
    let zen = zen_az
        .next()
        .unwrap()
        .strip_prefix("zen")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let az = zen_az.last().unwrap().parse::<u32>().unwrap();
    let mut sim_pointing = format!("zen{zen:02}az{az}");
    if CoordinateSystem::new(file, &sim_pointing).or_else(|_| {
        sim_pointing = format!("zen{zen:02}a{az}");
        CoordinateSystem::new(file, &sim_pointing).or_else(|_| {
            sim_pointing = format!("zen{zen:1}az{az}");
            CoordinateSystem::new(file, &sim_pointing).or_else(|_| {
                sim_pointing = format!("zen{zen:1}az{az}");
                CoordinateSystem::new(file, &sim_pointing)
            })
        })
    })? == CoordinateSystem::new(file, "TCS")?
    {
        // println!("PASS: {:<22} = TCS", sim_pointing);
        Ok(Check::Pass(("TCS", sim_pointing).into()))
    } else {
        // println!("FAIL: {:<22} ~ TCS", sim_pointing);
        // return Err(CfdCheckListError::CoordinateSystem(
        //     sim_pointing.to_string(),
        //     "TCS".to_string(),
        // ));
        Ok(Check::Fail(("TCS", sim_pointing).into()))
    }
}
pub fn check_tcs0<'a>(file: &'a str, case: &'a str) -> Result<Check<'a>, CfdCheckListError> {
    let zen_az = case.split("_").next().unwrap().split("az");
    let az = zen_az.last().unwrap().parse::<u32>().unwrap();
    let zenith_pointing = format!("zen0az{az}");
    if CoordinateSystem::new(file, &zenith_pointing)? == CoordinateSystem::new(file, "TCS0")? {
        // println!("PASS: {:<22} = TCS0", zenith_pointing);
        Ok(Check::Pass(("TCS0", zenith_pointing).into()))
    } else {
        // println!("FAIL: {:22} ~ TCS0", zenith_pointing);
        // return Err(CfdCheckListError::CoordinateSystem(
        //     zenith_pointing.to_string(),
        //     "TCS0".to_string(),
        // ));
        Ok(Check::Fail(("TCS0", zenith_pointing).into()))
    }
}
