/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

use std::fs;
use file_handler;
use logger;
use chrono::prelude::*;
use strings;

//todo: don't use pub config_file_path: String in this structure 
//todo: create new struct for data for process
//todo: or allow nill for config_file_path
#[derive(Deserialize,Serialize)]
pub struct SystemConfig 
{
    pub files: Files,
    pub config_file_path: String
}

#[derive(Deserialize,Serialize)]
pub struct Files {
    pub paths: Vec<String>
}


impl SystemConfig {
    pub fn new(contents: &str, iconfig_file_path: String) -> Option<SystemConfig> {
        //redo avoid duplication
        if !&contents.trim().is_empty() {
            let config: SystemConfig = toml::from_str(&contents).unwrap();
            Some(
                SystemConfig {
                    files: Files {
                        paths: strings::verify_input_path(config.files.paths)
                    },
                    config_file_path: iconfig_file_path
                        }
            )
        } else { None }
    }
}

impl PartialEq for Files {
    fn eq(&self, other: &Files) -> bool {
        self.paths == other.paths
    }
}

//Method to start watcher on the files specified in config files
pub fn process_config_files(config_files_path: &str) -> Vec<SystemConfig> {
    let paths = fs::read_dir(config_files_path).unwrap();
    let mut system_config_list: Vec<SystemConfig>  = Vec::new();
    for path in paths {
        let config_file = String::from(path.unwrap().path().to_str().unwrap());
        logger::log_info(&format!("read system config and start watcher for:  {}", config_file));
        let service_config = process_config_file(&config_file);
        //system_config_list.push(canonicalize(&service_config).map_err(|e| Error::Canonicalization(service_config, e))?);
        if service_config.is_some() {
            logger::log_debug(&format!("config file: {} is not empty", config_file));
            system_config_list.push(service_config.unwrap());
        } else {
            logger::log_info(&format!("Config file: {} is empty or in a wrong format", config_file));
        }
    }
    system_config_list
}

pub fn process_config_file(path: &str) -> Option<SystemConfig> {
    let content = &file_handler::read_from_file(&path);
    match content {
        Some(c) => {
            SystemConfig::new(c, String::from(path))
        },
        None => None,
    }
}

// // ****************  Don't remove it use for serialization of config obkect to file ****************  
// let system_config = SystemConfig {
//     files: Files {paths: vec!["firstpath1".to_string(), "60".to_string(), "arg".to_string()]},
//     recursive_directory: RecursiveDirectory {paths: vec!["firstpath2".to_string(), "60".to_string(), "arg".to_string()]},
//     directory: Directory {paths: vec!["firstpath3".to_string(), "60".to_string(), "arg".to_string()]},
// };
// let toml = toml::to_string(&system_config).unwrap();
// fs::write("/Users/olga.shafran@ibm.com/gitlab/anchor/conf.d/try_me1.toml", toml).expect("Unable to write file");
// // ****************  Don't remove it use for serialization of config obkect to file ****************  
