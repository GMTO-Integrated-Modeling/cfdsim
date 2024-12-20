use clap::Parser;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use cfd_checklist::{
    check_tcs, check_tcs0, match_report_to_case, CheckList, TestProperty, Tests, WindSpeed,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Full path to a CFD sim file
    #[arg(short, long)]
    case: String,
    /// Path to the CFD summary XML report
    #[arg(short, long)]
    report: Option<String>,
    /// Write checklist report to folder
    #[arg(short, long)]
    folder: bool,
}

fn checklist(case_path: &Path, cli: &Cli, root: PathBuf) -> anyhow::Result<()> {
    if case_path.is_dir() {
        println!("Applying checklist to all sim files in {case_path:?}");
        for entry in fs::read_dir(case_path)? {
            let path = entry?.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext != "sim" {
                        continue;
                    }
                }
            }
            checklist(
                path.as_path(),
                cli,
                root.join(case_path.file_name().unwrap()),
            )?;
        }
    } else {
        let case = case_path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        if cli.folder {
            if root.join(format!("{case}@PASS")).is_dir() {
                println!("found existing folder: {case}@PASS, skipping {case}");
                return Ok(());
            }
            if root.join(format!("{case}@FAIL")).is_dir() {
                println!("found existing folder: {case}@FAIL, skipping {case}");
                return Ok(());
            }
        };

        let report = if let Some(report) = cli.report.as_ref() {
            report
        } else {
            println!("Building report for {case} ...");
            let _output =
                Command::new("/opt/Siemens/17.06.007/STAR-CCM+17.06.007/star/bin/starccm+")
                    .args([
                        "-batch",
                        "-power",
                        "-podkey",
                        "wRCyd2S8PCE/6aG6NyauIw",
                        "-licpath",
                        "1999@flex.cd-adapco.com",
                        "/home/ubuntu/Desktop/report.java",
                    ])
                    .arg(case_path)
                    .output()
                    .expect(&format!("failed to build report for {case}"));
            println!(r#"{case} report saved in "/tmp/report.xml""#);
            // println!("{:#?}", output);
            "/tmp/report.xml"
        };
        match_report_to_case(report, &case)?;

        let wind_speed = WindSpeed::new(&case)?;
        let duration = wind_speed.duration();
        let start = wind_speed.start();
        let scene_start = wind_speed.scene_start();
        let u_max = wind_speed.u_max();

        println!("CHECKING {}...", case.to_uppercase());

        let tcs = check_tcs(report, &case)?;
        let tcs0 = check_tcs0(report, &case)?;

        let ducts = TestProperty::new(vec![("ducts", b"commonBoundary")], b"PartSurfaces", "")
            .check_ducts(report)?;
        let ws = TestProperty::new(vec![("ws", b"commonBoundary")], b"PartSurfaces", "")
            .check_ws(report)?;
        let instvol = TestProperty::new(vec![("instvol", b"commonBoundary")], b"PartSurfaces", "")
            .check_instvol(report)?;

        let test_props = vec![
            TestProperty::new(
                vec![("Umax", b"commonUserFieldFunction")],
                b"Definition",
                u_max,
            ),
            TestProperty::new(
                vec![(
                    "Maximum Physical Time",
                    b"commonPhysicalTimeStoppingCriterion",
                )],
                b"MaximumTime",
                duration,
            ),
            TestProperty::new(
                vec![("T_upwind", b"basereportSumReport")],
                b"Representation",
                "Volume Mesh",
            ),
            TestProperty::new(
                vec![("RI_tel", b"visScene"), ("Scalar 1", b"visScalarDisplayer")],
                b"Representation",
                ["Volume Mesh", "Latest Surface/Volume"],
            ),
            TestProperty::new(
                vec![("RI_tel", b"visScene"), ("Update", b"visSceneUpdate")],
                b"DeltaTime",
                "0.2 s",
            ),
            TestProperty::new(
                vec![("RI_tel", b"visScene"), ("Update", b"visSceneUpdate")],
                b"StartQuantity",
                scene_start,
            ),
            TestProperty::new(
                vec![
                    ("RI_wind", b"visScene"),
                    ("Scalar 1", b"visScalarDisplayer"),
                ],
                b"Representation",
                ["Volume Mesh", "Latest Surface/Volume"],
            ),
            TestProperty::new(
                vec![("RI_wind", b"visScene"), ("Update", b"visSceneUpdate")],
                b"DeltaTime",
                "0.2 s",
            ),
            TestProperty::new(
                vec![("RI_wind", b"visScene"), ("Update", b"visSceneUpdate")],
                b"StartQuantity",
                scene_start,
            ),
            TestProperty::new(
                vec![
                    ("vort_tel", b"visScene"),
                    ("Scalar 1", b"visScalarDisplayer"),
                ],
                b"Representation",
                ["Volume Mesh", "Latest Surface/Volume"],
            ),
            TestProperty::new(
                vec![("vort_tel", b"visScene"), ("Update", b"visSceneUpdate")],
                b"DeltaTime",
                "0.2 s",
            ),
            TestProperty::new(
                vec![("vort_tel", b"visScene"), ("Update", b"visSceneUpdate")],
                b"StartQuantity",
                scene_start,
            ),
            TestProperty::new(
                vec![
                    ("vort_wind", b"visScene"),
                    ("Scalar 1", b"visScalarDisplayer"),
                ],
                b"Representation",
                ["Volume Mesh", "Latest Surface/Volume"],
            ),
            TestProperty::new(
                vec![("vort_wind", b"visScene"), ("Update", b"visSceneUpdate")],
                b"DeltaTime",
                "0.2 s",
            ),
            TestProperty::new(
                vec![("vort_wind", b"visScene"), ("Update", b"visSceneUpdate")],
                b"StartQuantity",
                scene_start,
            ),
            TestProperty::new(
                vec![("M1p", b"commonXyzInternalTable")],
                b"Representation",
                "Volume Mesh",
            ),
            TestProperty::new(
                vec![
                    ("M1p", b"commonXyzInternalTable"),
                    ("Update", b"commonTableUpdate"),
                ],
                b"DeltaTime",
                "0.05 s",
            ),
            TestProperty::new(
                vec![
                    ("M1p", b"commonXyzInternalTable"),
                    ("Update", b"commonTableUpdate"),
                ],
                b"StartQuantity",
                start,
            ),
            TestProperty::new(
                vec![("M2p", b"commonXyzInternalTable")],
                b"Representation",
                "Volume Mesh",
            ),
            TestProperty::new(
                vec![
                    ("M2p", b"commonXyzInternalTable"),
                    ("Update", b"commonTableUpdate"),
                ],
                b"DeltaTime",
                "0.05 s",
            ),
            TestProperty::new(
                vec![
                    ("M2p", b"commonXyzInternalTable"),
                    ("Update", b"commonTableUpdate"),
                ],
                b"StartQuantity",
                start,
            ),
            TestProperty::new(
                vec![("optvol", b"commonXyzInternalTable")],
                b"Representation",
                "Volume Mesh",
            ),
            TestProperty::new(
                vec![
                    ("optvol", b"commonXyzInternalTable"),
                    ("Update", b"commonTableUpdate"),
                ],
                b"DeltaTime",
                "0.2 s",
            ),
            TestProperty::new(
                vec![
                    ("optvol", b"commonXyzInternalTable"),
                    ("Update", b"commonTableUpdate"),
                ],
                b"StartQuantity",
                start,
            ),
            TestProperty::new(
                vec![("Telescope_p", b"commonXyzInternalTable")],
                b"Representation",
                "Volume Mesh",
            ),
            TestProperty::new(
                vec![
                    ("Telescope_p", b"commonXyzInternalTable"),
                    ("Update", b"commonTableUpdate"),
                ],
                b"DeltaTime",
                "0.05 s",
            ),
            TestProperty::new(
                vec![
                    ("Telescope_p", b"commonXyzInternalTable"),
                    ("Update", b"commonTableUpdate"),
                ],
                b"StartQuantity",
                start,
            ),
            TestProperty::new(
                vec![("AMG Linear Solver", b"commonAMGLinearSolver")],
                b"CycleOption",
                "AMGCycleOption.V_CYCLE",
            ),
        ];
        let mut checklist = CheckList::try_from(Tests::new(report, test_props))?;
        checklist.push(tcs);
        checklist.push(tcs0);
        checklist.push(ducts);
        checklist.push(ws);
        checklist.push(instvol);

        let folder_path = if cli.folder {
            let folder = format!(
                "{case}@{}",
                checklist.pass().then_some("PASS").unwrap_or("FAIL")
            );
            let folder_path = root.join(folder);
            println!("Writing checklist to {folder_path:?}");
            fs::create_dir_all(&folder_path)?;
            let mut file = File::create(folder_path.join("checklist.txt"))?;
            write!(&mut file, "CHECKING {:}", case.to_lowercase())?;
            write!(&mut file, "{checklist}")?;
            Some(folder_path)
        } else {
            println!("{checklist}");
            None
        };

        if checklist.pass() {
            println!("Writing RI_tel, RI_wind, vort_tel, vort_wind hardcopies ...");
            let _output =
                Command::new("/opt/Siemens/17.06.007/STAR-CCM+17.06.007/star/bin/starccm+")
                    .args([
                        "-batch",
                        "-power",
                        "-podkey",
                        "wRCyd2S8PCE/6aG6NyauIw",
                        "-licpath",
                        "1999@flex.cd-adapco.com",
                        "/home/ubuntu/Desktop/scenes_views.java",
                    ])
                    .arg(case_path)
                    .output()
                    .expect(&format!("failed to build report for {case}"));
            for scene in ["RI_tel", "RI_wind", "vort_tel", "vort_wind"] {
                let root = Path::new(env!("HOME")).join("Desktop");
                if let Err(e) = fs::rename(
                    root.join(format!("{scene}.png")),
                    if let Some(ref folder) = folder_path {
                        folder.join(format!("{scene}.png"))
                    } else {
                        root.join(format!("{case}_{scene}.png"))
                    },
                ) {
                    println!("failed to generate scene view hardcopies {e}");
                }
            }
        }
    }
    Ok(())
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let case_path = Path::new(&cli.case);
    let root = Path::new(env!("HOME")).join("Desktop");
    checklist(case_path, &cli, root)?;
    Ok(())
}