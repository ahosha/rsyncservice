
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */


use logger;
use rsync_sync;
use pre_check_runner;
use const_enum;



fn pre_check(qradar_integrated: bool, remote_ip : String) -> bool {
    logger::log_debug("-----------perform pre_check");
    if !qradar_integrated {
        true
    } else {
        pre_check_runner::run_checkers(false, remote_ip)
    }
}

// fn prepare_paths(destination_ip : &str) -> ( Vec<String> , bool){
//     logger::log_debug("-------- prepare_paths");
//     let paths_list = paths::Paths::created();
//     let self_config_paths_list = paths::Paths::created();
//     let local_changes = paths_operations::collect_paths( &paths_list );
//     let path_for_sync = paths_operations::convert_to_vec(&paths_list);
//     (path_for_sync, local_changes)
// }

pub fn run_rsync (qradar_integrated: bool,
                  destination_ip :&str,
                  path_for_sync: Vec<String>,
                  destination_path: &str,
                  rate: i32)  -> const_enum::ManageMode
{
   let mut _result = const_enum::ManageMode::Start;
    if  pre_check(qradar_integrated, destination_ip.to_string()) {
        logger::log_debug("-----------continue to sync action");
        //TODO: implement rsync_sync::get_exclude_param : "$(get_local_state)" == "active" ....
        let exclude_param = rsync_sync::get_exclude_param(qradar_integrated);
        //TODO: add all mode support  RSyncMode
        rsync_sync::rsync_command_performer(&path_for_sync, &destination_path,
                                            &rate.to_string(),
                                            &rsync_sync::RSyncMode::File,
                                            exclude_param);
        logger::log_debug("----------rsync_command_performer done");
        _result = const_enum::ManageMode::Done;
    } else {
        logger::log_debug("-----------pre check failed");
        _result = const_enum::ManageMode::Retransmission;
    }
    _result
}