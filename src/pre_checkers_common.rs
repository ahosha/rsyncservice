
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

///
/// Prechecks implementation to check various processes on Qradar(HA) that would be needed for rsync service to perform rsync
///
use std::process::Command;
use logger;
use environment;
use command_parser;
use const_name;

pub trait Checker {
    fn check(&self) -> bool;
}


pub fn check_qradar_status (cmd: &str, remote_ip : &str) -> bool{
        let mut _passed = true;
        let _output = Command::new(cmd)
            .arg(remote_ip)
            .output().unwrap_or_else(|e| panic!("failed to execute process {}: {}",
                                                const_name::QRADAR_ENVIRONMENT.to_owned() + "bin/getstatus.sh", e));
        let status_msg = String::from_utf8_lossy(&_output.stdout);
        if _output.status.success() {
            if status_msg == const_name::MESSAGE_NEED_UPDATE {
                logger::log_debug(&format!("QradarStatusChecker received {} from  {} -> return false",
                                           const_name::MESSAGE_NEED_UPDATE,
                                           const_name::QRADAR_ENVIRONMENT.to_owned() + "bin/getstatus.sh"));
                _passed = false;
            }
        } else {
            logger::log_debug(&format!("QradarStatusChecker failed run {} because of: {} ",
                                       const_name::QRADAR_ENVIRONMENT.to_owned() + "bin/getstatus.sh",
                                       status_msg));
            _passed = false;
        }
        _passed
}

pub fn check_ha_setup_is_running() -> bool {
        let mut _passed = true;
        let _output = Command::new(const_name::PID_SCRIPT)
            .arg(const_name::PID_SCRIPT_EXISTS_ARG)
            .arg(const_name::HA_SETUP_SCRIPT)
            .output().unwrap_or_else(|e| panic!("failed to execute process{}: {}",
                                                const_name::PID_SCRIPT, e));

        let status_msg=String::from_utf8_lossy(&_output.stdout);
        if _output.status.success() {
                //todo: retrun status from precheckers not just bool value
                logger::log_debug("An instance of the HA setup script is running. Skippimg with retry -> return false");
                _passed = false;
        }
        else {
            logger::log_debug(&format!("HASetupStatusChecker failed run {} because of: {} ",
                                       const_name::PID_SCRIPT,
                                       status_msg));
            _passed = false;
        }
        _passed
}

//TODO: add usage _remote
pub fn get_ha_state(cmd: &str, _remote: bool) -> bool {
    let mut _passed = true;
    let _output = Command::new(cmd)
            .arg(const_name::HA_SCRIPT_REMOTE_ARG)
            .output().unwrap_or_else(|e| panic!("failed to execute process{}: {}",
                                                cmd, e));
    let status_msg=String::from_utf8_lossy(&_output.stdout);
    if _output.status.success() {
        if status_msg.is_empty() {
            logger::log_debug("Failed to get system status.  Skippimg with retry -> return false");
            _passed = false;
        } else {
            let status = status_msg.trim().to_lowercase();
            match  status.as_ref() {
                "standby" | "offline" => {
                    logger::log_debug(&format!("Remote system status is {} .  Skippimg with retry -> return false",
                                               status_msg.trim().to_lowercase()));
                    _passed = false;
                },
                _ => _passed = true,
            }
        }
    } else {
        logger::log_debug(&format!("HARemoteStateChecker failed run {} because of: {} ",
                                   const_name::PID_SCRIPT,
                                   status_msg));
        _passed = false;
    }
    _passed
}

pub fn get_ha_remote_state (cmd: &str, ha_skip_remote_state_rsync_check: &str) -> bool {
    if !environment::path_exists(&ha_skip_remote_state_rsync_check) {
        get_ha_state(cmd, true)
    } else {
        //skip
        logger::log_debug(&format!("Detected {} token.  Removing token and skipping remote state check.",
                                   ha_skip_remote_state_rsync_check));
      
        remove_file(ha_skip_remote_state_rsync_check);
        true
    }
}

pub fn remove_file(path: &str) -> bool {
    let _output = Command::new("rm")
        .arg("-f")
        .arg(path)
        .spawn()
        .expect("rm command failed to start");
  
    true
}

//TODO: add remote call
pub fn check_host_version (cmd: &str) -> bool{
    let mut _passed = true;
    let _output = Command::new(cmd)
        .output().unwrap_or_else(|e| panic!("failed to execute process {}: {}", cmd, e));
    let status_msg = String::from_utf8_lossy(&_output.stdout);
    if _output.status.success() {
        let host_version = command_parser::extract_host_version(&status_msg);
        let output_remote = Command::new(cmd)
            .output().unwrap_or_else(|e| panic!("failed to execute process {}: {}", cmd, e));
        let status_msg_remote = String::from_utf8_lossy(&output_remote.stdout);
        if output_remote.status.success() {
            let host_version_remote = command_parser::extract_host_version(&status_msg_remote);
            if host_version == host_version_remote {
                _passed = true;
            } else {
                logger::log_debug(&format!("Remote system is version {} but we are {}. Skippimg with retry -> return false",
                                           host_version_remote, host_version ));
                _passed = false;
            }
        }
    } else {
        logger::log_debug(&format!("check_host_version failed run {} because of: {} ",
                                   const_name::QRADAR_ENVIRONMENT.to_owned() + "bin/getstatus.sh",
                                   status_msg));
        _passed = false;
    }
    _passed
}

//TODO: add remote call
pub fn check_qradar_build_number (cmd: &str) -> bool{
    let mut _passed = true;
    //run command locally
    let _output = Command::new(cmd)
        .output().unwrap_or_else(|e| panic!("failed to execute process {}: {}", cmd, e));
    let status_msg = String::from_utf8_lossy(&_output.stdout);
    if _output.status.success() {
        let local_build_number = &status_msg.trim().to_lowercase();
        //run command remotlly
        //TODO: add remote call ability
        let output_remote = Command::new(cmd)
            .output().unwrap_or_else(|e| panic!("failed to execute process {}: {}", cmd, e));
        let status_msg_remote = String::from_utf8_lossy(&output_remote.stdout);
        if output_remote.status.success() {
            let local_build_number_remote = &status_msg_remote.trim().to_lowercase();
            if local_build_number == local_build_number_remote {
                _passed = true;
            } else {
                logger::log_debug(&format!("Remote system is build version {} but we are {}. Skippimg with retry -> return false",
                                           local_build_number_remote, local_build_number ));
                _passed = false;
            }
        }
    } else {
        logger::log_debug(&format!("check_qradar_build_number failed run {} because of: {} ",
                                   const_name::QRADAR_ENVIRONMENT.to_owned() + "bin/getstatus.sh",
                                   status_msg));
        _passed = false;
    }
    _passed
}



