// use environment;
// use const_name;

// use std::process::Command;



// // pub fn  get_local_state() -> String{
// //     let script_to_run = format!("{}{}",const_name::HA_ROOT_DIR, const_name::HA_GETSTATE_SCRIPT_PATH);
// //     if environment::path_exists(&script_to_run) {
// //         let output = Command::new(script_to_run)
// //             .output()
// //             .expect("failed to execute process");
// //         String::from(String::from_utf8_lossy(&output.stdout).trim())
// //     } else { String::from("n/a" )}
// // }

// // pub fn  restart_rsyncservice() {
// //     Command::new("systemctl")
// //         .arg("restart")
// //         .arg("rsyncservice")
// //         .spawn()
// //         .expect("systemctl restart rsyncservice command failed to start");
// // }

