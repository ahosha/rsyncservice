/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

use std::fs;
///
/// Service configurator looks for config file in project and provides the rsync service with destination IP
/// And files to be watched upon
/// 
#[derive(Deserialize,Serialize)]
pub struct ServiceConfig {
    pub destination: Destination,
    pub config: Config
}

#[derive(Deserialize,Serialize)]
pub struct Destination {
    pub ip: String
}


#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub config_folder: String,
    pub qradar_integrated: bool
}

//Method to read the config file contents and provide the destination IP and files to be watched up on to rsync service
 impl ServiceConfig {
    pub fn new(contents: &str) -> Option<ServiceConfig> {
        //redo avoid duplication
        if !&contents.trim().is_empty() {
                   

            let service_config: Result<ServiceConfig, ::toml::de::Error> = toml::from_str(&contents);
                        
            match service_config {
                
                Err(_e) => None,
                
                Ok(v) => {
                    println!("check2");
                    Some( ServiceConfig {
                        destination: Destination { ip: v.destination.ip },
                        config: Config { config_folder: v.config.config_folder,
                            qradar_integrated: v.config.qradar_integrated }
                           })
                    }
                  
            }
              
        } else { None }
    }


    pub fn compose_new (dest_ip: String, config_folder: String, qradar_integrated: bool) -> ServiceConfig {

        ServiceConfig {
            destination: Destination { ip: dest_ip },
            config: Config {  config_folder,
                 qradar_integrated }
        }
    }
}

impl PartialEq for Destination {
    fn eq(&self, other: &Destination) -> bool {
        self.ip == other.ip
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Config) -> bool {
        self.config_folder == other.config_folder
    }
}
//Method for serialization of config object to file
pub fn serialization_object_file() { 
    let service_config = ServiceConfig {
   destination: Destination {ip: "Destination_IP".to_string()},
    
    config: Config {config_folder: "config_folder=/etc/anchor/conf.d".to_string(), qradar_integrated: false},
 };
let toml = toml::to_string(&service_config).unwrap();
 fs::write("File to be tracked", toml).expect("Unable to write file");
}

//// ****************  Don't remove it use for serialization of config obkect to file ****************
//let service_config = ServiceConfig {
//destination: Destination {ip: "172.18.216.56".to_string()},
//config: Config {config_folder: "/etc/anchor/conf.d".to_string(),
//qradar_integrated: false },
//};
//let toml = toml::to_string(&service_config).unwrap();
//fs::write("/Users/olga.shafran@ibm.com/gitlab/anchor-refactoring/conf/anchor.toml", toml).expect("Unable to write file");
//// ****************  Don't remove it use for serialization of config obkect to file ****************

