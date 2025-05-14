/*!
# BATCH MODE FOR STARCCM+ MACROS

Apply a java macro to a collection of StarCCM+ sim files
*/

use std::{fs, path::Path};

use cfdsim::Macro;

const JAVA: &str = "/home/ubuntu/Desktop/ForceChange/ForceChangeAll.java";

fn main() -> anyhow::Result<()> {
    let cases_path = Path::new("/home/ubuntu/mnt/sims_ready/");

    for cg in ["OS_2ms", "OS_7ms", "CD_12ms", "CD_17ms"] {
        for az in [0, 45, 90, 135, 180] {
            let case = format!("zen30az{az:03}_{cg}");
            let case_path = cases_path.join(&case).with_extension("sim");

            let case_update_path = Path::new("/home/ubuntu/mnt/sims_update/")
                .join(case)
                .with_extension("sim");
            if case_update_path.exists() {
                println!("case {case_update_path:?} already exists, skipping");
                continue;
            }

            println!("updating {case_path:?} ...");
            Macro::new(&case_path, JAVA)?.play()?;
            println!(
                r#"copying "/home/ubuntu/Desktop/sim_force_update.sim" to {case_update_path:?} ..."#
            );
            fs::copy(
                "/home/ubuntu/Desktop/sim_force_update.sim",
                case_update_path,
            )?;
        }
    }
    Ok(())
}
