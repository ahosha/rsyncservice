/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

///
/// This class sets the default environment variable related to config files for rsync service
/// 
use std::env;
use std::fs;
use std::path::PathBuf;
use std::io;
use const_name;
use logger;


pub fn env_is_set_value(var_name: &str) -> String {
    match env::var(var_name) {
        Ok(s) => s,
        _ => String::from("")
    }
}


pub fn path_exists(path: &str) -> bool {
    logger::log_debug(&format!("check path_exists {}", path));
    fs::metadata(path).is_ok()
}


// pub fn get_service_conf_file_path() -> io::Result<PathBuf>  {
//     let mut dir = env::current_exe()?;
//     dir.pop();
//     dir.push("conf");
//     dir.push("anchor.toml");
//     Ok(dir)
// }

// pub fn get_default_subsystem_conf_path() -> io::Result<PathBuf>  {
//     let mut dir = env::current_exe()?;
//     dir.pop();
//     dir.push("conf.d");
//     Ok(dir)
// }


pub fn get_exe_path() -> Result<PathBuf, io::Error> {
    env::current_exe()
}

//redo - use static ??
// static SERVICE_CONFIG_FILE_LOCATION: String = "/etc/ha/anchor".to_string();
// rsync_config_file = SERVICE_CONFIG_FILE_LOCATION;
pub fn get_service_local_config_file_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("conf");
    dir.push("anchor.toml");
    Ok(dir)
}


pub fn get_service_config_file_path() -> io::Result<PathBuf> {
    let mut path = PathBuf::new();
    path.push(r"/");
    path.push("etc");
    path.push("ha");
    path.push("anchor");
    path.push("anchor.toml");
    Ok(path)
}


// pub fn get_cmd_to_run() -> io::Result<PathBuf> {
//     let mut dir = env::current_exe()?;
//     dir.pop();
//     //TODO: get dynamicaky command to run from config file
//     dir.push("syncservice");
//     Ok(dir)
// }


pub fn get_subsystem_config_file_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("conf.d");
    Ok(dir)
}



pub fn get_mock_file_path(script_name: &str) -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push(const_name::MOCK_SCRIPT_ROOT_FOLDER);
    dir.push(script_name);
    Ok(dir)
}


