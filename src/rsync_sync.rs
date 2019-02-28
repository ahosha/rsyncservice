
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

use command_parser;
use const_name;
use environment;
use file_handler;
use logger;
use std::process::Command;

pub enum RSyncMode {
    File,
    Dir,
    DirRec,
}

fn get_rate() -> i32 {
    if environment::path_exists(const_name::DRBD_CONF_FILE_NAME) {
        let config_content = &file_handler::read_from_file(const_name::DRBD_CONF_FILE_NAME);
        match config_content {
            Some(c) => {
                if !c.trim().is_empty() {
                    command_parser::get_sync_rate(c)
                } else {
                    0
                }
            }
            None => 0,
        }
    } else {
        0
    }
}

fn get_destination_path(destination_ip: &str) -> String {
    if destination_ip.trim().is_empty() {
        let mut path = environment::get_exe_path().expect(const_name::MESSAGE_GEN_ERROR);
        path.pop();
        format!(
            "{}/{}",
            &path.to_str().unwrap().to_string(),
            "rsync_local_sync_dir"
        )
    } else {
        format!("{}{}", destination_ip, ":/")
    }
}

pub fn get_exclude_param(_qradar_integrated: bool) -> String {
    String::from("")
}

//TODO: return struct
pub fn prepare_parameter_for_sync(destination_ip: &str) -> (i32, String) {
    let rate = get_rate();
    let  destination_path = get_destination_path(destination_ip);
    
    (rate, destination_path)
}

pub fn rsync_command_performer(
    paths_args: &[String],
    destination_path: &str,
    _rsync_rate: &str,
    mode: &RSyncMode,
    _exclude_param: String,
) {
    logger::log_debug(&format!(
        "rsync_command_performer -> {:?} destination_path: {}",
        &paths_args, destination_path
    ));
    if !paths_args.is_empty() {
        let cmd = String::from("rsync");
        let cmd_param;
        match mode {
            RSyncMode::File => cmd_param = String::from("-avRK"),
            RSyncMode::Dir => cmd_param = String::from("-avzh"),
            RSyncMode::DirRec => cmd_param = String::from("-avRKzh"),
        }
        Command::new(&cmd)
            .arg(cmd_param)
            .arg("--delete")
            .arg("--ignore-errors")
            .args(paths_args)
            .arg(destination_path)
            .spawn()
            .expect("rsync command failed to start");
    }
}
