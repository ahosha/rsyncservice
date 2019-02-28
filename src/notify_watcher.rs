/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

///
/// The class provides implementation of retransmission and watcher of rsync service. 
/// Also, it keeps a track of events noticed during service execution
///  
extern crate notify;
use std::fs::canonicalize;
use logger;
use rsync_sync;
use paths_operations;
use rsync_sync_processor;
use const_enum;
use notification_channel;


use notify::{RecommendedWatcher, Watcher, RecursiveMode, RawEvent, raw_watcher};


use std::env;
use std::sync::mpsc::{channel};


fn retransmission(destination_ip: &str,
                  path: &str,
                  destination_path: &str,
                  rate: i32) -> bool {
    logger:: log_debug(&format!("start retransmission destination_ip:{} path:{} destination_path:{} rate:{}",
                                destination_ip, path, destination_path, rate));
    let sync_queue= vec![path.to_string()];
    let run_result = rsync_sync_processor::run_rsync(false,
                                                     &destination_ip,
                                                     sync_queue,
                                                     &destination_path,
                                                     rate);
    //TODO: add error handling
    match run_result {

        const_enum::ManageMode::Retransmission => {
            logger::log_debug(&format!("run_result MANAGE_MSG_RETRANSMISSION"));
            true
        },
        _ => {logger::log_debug(&format!("notify_watcher -> watch-> run_result received not Retransmission"));
            false},
    }
}
// Rsync watcher implementation, keeps a track of files which are configured for rsync service
pub fn watch(paths_collection: Vec<String>,
             destination_ip: &str,
             sync_notification_channel: &notification_channel::NotificationChannel,
             service_config_folder: &str) -> const_enum::ManageMode {
    logger:: log_debug(&format!("watch: destination_ip {} current exe path: {} ",
                                destination_ip,
                                env::current_exe().unwrap().to_str().unwrap()));
    let (_tx, _rx) = channel();
    let mut watcher = raw_watcher(_tx).unwrap();



    let mut paths = vec![];
    for path in paths_collection {
        //TODO: add error handling for wromg path
        paths.push(canonicalize(&path).unwrap());
    }

    for path in paths {
        //try!(watcher.watch(path,RecursiveMode::Recursive));
          watcher.watch(path,RecursiveMode::Recursive);
    }
    loop {
        match _rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
                logger:: log_debug(&format!("RawEvent  {:?} ({:?}) ({:?}) ({:?})",
                                            op, op.bits(), path, cookie));
                let mut is_on_remove = false;
                if !op.is_empty() {
                    let opbits = op.bits();
                    match opbits {
                        4 => is_on_remove = true,
                        _ => logger::log_debug(&format!("received all other events , not REMOVE")),
                    }
                }
                let notify_path = path.to_str().unwrap().to_string();
                sync_notification_channel.add(notify_path.to_string());
                // *************************************run_rsync********************************************************
                let sync_queue = sync_notification_channel.get_inner_vector();
                logger::log_debug(&format!("!!! worker messages in sync queue:{} !!!", sync_queue.len()));
                sync_notification_channel.clear();
                let (rate, destination_path) = rsync_sync::prepare_parameter_for_sync(&destination_ip);
                logger::log_debug("iterate through sync_queue -> check if local config changes are done");
                let mut _local = false;
                let mut parent_queue = Vec::new();
                for path in sync_queue.iter() {
                    _local = paths_operations::is_local_config_changes(path, service_config_folder);
                    if _local {
                        logger::log_debug("local changes . will RESTART.");
                        return const_enum::ManageMode::Restart
                    }
                    if is_on_remove {
                        parent_queue.push(paths_operations::get_parent_path_string(&*path).to_str().unwrap().to_string());
                    }
                }

                logger::log_debug(&format!("start rsync from NOTIFY_WATCHER"));
                let mut _run_result = const_enum::ManageMode::Done;
                if !is_on_remove {
                    _run_result = rsync_sync_processor::run_rsync(false,
                                                                 &destination_ip,
                                                                 sync_queue,
                                                                 &destination_path,
                                                                 rate);
                } else {
                    _run_result = rsync_sync_processor::run_rsync(false,
                                                                 &destination_ip,
                                                                 parent_queue,
                                                                 &destination_path,
                                                                 rate);
                }
                //TODO: add error handling
                match _run_result {
                    const_enum::ManageMode::Retransmission => {
                        logger::log_debug(&format!("run_result MANAGE_MSG_RETRANSMISSION"));
                        let mut run_retransmission = true;
                        let path = &*destination_path;
                        for _x in 0..3 {
                            if run_retransmission {
                                run_retransmission = retransmission(destination_ip,
                                                                    &*notify_path,
                                                                    path,
                                                                    rate);
                            }
                        }
                    },
                    const_enum::ManageMode::Restart => {
                        logger::log_debug(&format!("run_result  NEED RESTART"));
                    },
                    const_enum::ManageMode::Done => {
                        logger::log_debug(&format!("run_result  Done"));
                    },
                    const_enum::ManageMode::Start => {
                        logger::log_debug(&format!("run_result  Start"));
                    },
                    _ => { logger::log_debug(&format!("notify_watcher -> watch-> run_result received not Retransmission, not Restart, not Done, not Start")) },
                }
                // *************************************run_rsync********************************************************
            },

            Ok(event) => {logger:: log_debug(&format!("broken event: {:?}", event))},

            Err(e) => {logger:: log_debug(&format!("watch error: {:?}", e))},
        }

    }
}
