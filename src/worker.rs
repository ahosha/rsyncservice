
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

///Starting point of Rsync Service
/// Service looks for config file in directory or as an environment variable
/// And after finding the config file it reads the destination IP and files to watch for sync
use std::io;
use std::io::{Error, ErrorKind};

use std::sync::mpsc::{channel, Sender};
use std::thread;

use const_enum;
use const_name;
use environment;
use error_handler;
use file_handler;
use logger;
use notification_channel;
use notify_watcher;
use rsync_sync;
use service_toml_configurator::ServiceConfig;
use subsystem_toml_configurator;
use subsystem_toml_configurator::SystemConfig;

//Method looks for Config file in root directory or as an environment variable

fn process_configs() -> Result<(Vec<SystemConfig>, ServiceConfig), io::Error> {
    let mut rsync_config_file =
        environment::env_is_set_value(const_name::ENV_VAR_NMAE_RSYNC_CONFIG_FILE);
    if rsync_config_file.is_empty() {
        let mut default_path =
            environment::get_service_config_file_path().expect(const_name::MESSAGE_GEN_ERROR);
        if !environment::path_exists(&default_path.to_str().unwrap()) {
            logger::log_debug(&format!(
                "config file:{} doesn't exists",
                default_path.to_str().unwrap()
            ));
            default_path = environment::get_service_local_config_file_path()
                .expect(const_name::MESSAGE_GEN_ERROR);
            if !environment::path_exists(&default_path.to_str().unwrap()) {
                logger::log_debug(&format!(
                    "config file:{} doesn't exists",
                    default_path.to_str().unwrap()
                ));
                error_handler::fail_log_handler("service config file doesn't exists. Exit");
            }
        }
        rsync_config_file = String::from(default_path.to_str().unwrap());
        let warn_message = format!(
            "environtment variable rsync_config_file was not configured. read from file: {}",
            rsync_config_file
        );
        error_handler::warn_log_handler(&warn_message);
    }
//Checks the config file for the destination IP and files to work upon for rsync
    let config_file_content = &file_handler::read_from_file(&rsync_config_file);
    match config_file_content {
        Some(c) => {
            logger::log_debug("try to compose ServiceConfig object");
            let service_config = ServiceConfig::new(c);
            match service_config {
                Some(conf_obj) => {
                    let mut subsystem_config_folder = conf_obj.config.config_folder.clone();
                    if environment::path_exists(&subsystem_config_folder) {
                        logger::log_debug(&format!(
                            "config folder:{} exists",
                            subsystem_config_folder
                        ));
                    } else {
                        let path = environment::get_subsystem_config_file_path()
                            .expect(const_name::MESSAGE_GEN_ERROR);
                        let error_message = format!(
                            "config folder:{0} doest not exists. use default value: {1}/",
                            subsystem_config_folder,
                            String::from(path.to_str().unwrap())
                        );
                        error_handler::warn_log_handler(&error_message);
                        subsystem_config_folder = String::from(path.to_str().unwrap());
                    }
                    //Checks for the path of files to be tracked for rsync service
                    let monitoring_entities =
                        subsystem_toml_configurator::process_config_files(&subsystem_config_folder);
                    for entity in monitoring_entities.iter() {
                        for file in entity.files.paths.iter() {
                            if !environment::path_exists(file) {
                                logger::log_debug(&format!(
                                    "path:{} doesn't exists. won't be monitored",
                                    file
                                ));
                                error_handler::fail_log_handler(&format!(
                                    "wrong configuration file {}. Exit",
                                    file
                                ));
                            }
                        }
                    }
                   //Positive path- files are good to be monitiored 
                    Ok((
                        monitoring_entities,
                        ServiceConfig::compose_new(
                            conf_obj.destination.ip,
                            subsystem_config_folder.to_string(),
                            conf_obj.config.qradar_integrated,
                        ),
                    ))
                }
                
                None => {
                    let error = &format!("corrupted config file: {}", &rsync_config_file);
                    let custom_error = Error::new(ErrorKind::Other, error.to_string());
                    Err(custom_error)
                }
            }
        }
        //Negative path- files have some issues
        None => {
            let error = &format!(
                "error while reading from config file: {}",
                &rsync_config_file
            );
            let custom_error = Error::new(ErrorKind::Other, error.to_string());
            Err(custom_error)
        }
    }
}
//Adding multiple file path for tracking
fn concat_paths(monitoring_entities: &[SystemConfig]) -> Vec<String> {
    let mut concatenated_paths_list = vec![];

    for systemconfig in monitoring_entities.iter() {
        concatenated_paths_list.extend(systemconfig.files.paths.iter().cloned());
    }
    concatenated_paths_list
}

//TODO: add watcher factory
//Intialization of Rsync process
fn process(
    monitoring_entities: &[SystemConfig],
    mode: &const_enum::RunOptions,
    service_config: &ServiceConfig,
) -> const_enum::ManageMode {
    let destination_ip = service_config.destination.ip.to_string();
    let config_folder = service_config.config.config_folder.to_string();
    logger::log_debug(&format!(
        "first service_config.config.config_folder: {}",
        config_folder
    ));
    let concatenated_paths_list = concat_paths(monitoring_entities);
    let mut managedresult = const_enum::ManageMode::Passed;
    match mode {
        const_enum::RunOptions::All => {
            logger::log_debug("Starting initial sync");
            let (rate, destination_path) = rsync_sync::prepare_parameter_for_sync(&destination_ip);
            rsync_sync::rsync_command_performer(
                &concatenated_paths_list,
                &destination_path,
                &rate.to_string(),
                &rsync_sync::RSyncMode::File,
                String::from(""),
            );
            logger::log_debug(&format!(
                "notify_watcher Starting watching destination_ip:{}",
                &destination_ip
            ));
            let sync_notification_channel = notification_channel::NotificationChannel::created();
            logger::log_debug(&format!(
                "service_config.config.config_folder: {}",
                service_config.config.config_folder
            ));
            let result = notify_watcher::watch(
                concatenated_paths_list,
                &destination_ip,
                &sync_notification_channel,
                &*service_config.config.config_folder,
            );

            match result {
                const_enum::ManageMode::Retransmission => {
                    logger::log_debug(&format!("run_result MANAGED_MSG_RETRANSMISSION"));
                    managedresult = const_enum::ManageMode::Retransmission;
                }
                const_enum::ManageMode::Restart => {
                    logger::log_debug(&format!("from WORKER: result  NEED RESTART"));
                    managedresult = const_enum::ManageMode::Restart;
                }
                _ => logger::log_debug(&format!(
                    "worker -> process-> run_result received not Retransmission, not Restart"
                )),
            }
        }
    }
    managedresult
}

fn process_all(
    monitoring_entities: &[SystemConfig],
    service_config: &ServiceConfig,
) -> const_enum::ManageMode {
    process(
        monitoring_entities,
        &const_enum::RunOptions::All,
        service_config,
    )
}
//Thread implementation for rsync service commands Start, Restart, Retransmission and Stop
pub fn spawn_worker(tx_main: Sender<const_enum::ManageMode>) -> Sender<const_enum::ManageMode> {
    let (_tx, _rx) = channel();
    thread::spawn(move || {
        loop {
            let msg = _rx.recv().unwrap();
            match msg {
                const_enum::ManageMode::Start => {
                    logger::log_debug("The worker is starting.");
                    let read_config_result = process_configs();
                    match read_config_result {
                        Ok((monitoring_entities, service_config)) => {
                            logger::log_debug(&format!(
                                "spawn_worker service_config.config.config_folder: {}",
                                service_config.config.config_folder
                            ));
                            let runresult = process_all(&monitoring_entities, &service_config);
                            match runresult {
                                const_enum::ManageMode::Restart => {
                                    logger::log_debug("spawn_worker received Restart");
                                    break
                                },
                                const_enum::ManageMode::Passed  => {
                                    logger::log_debug(&format!("spawn_worker Passed" ));
                                },
                                const_enum::ManageMode::GeneralFail(runresult)  => {
                                    logger::log_debug(&format!("*******spawn_worker GeneralFail****** :{}", runresult ));
                                },
                                const_enum::ManageMode::Retransmission  => {
                                    logger::log_debug(&format!("spawn_worker Retransmission" ));
                                    //TODO: add retramsmission  OLGA
                                },
                                _ => { logger::log_debug("spawn_worker -> Start flow received not Restart, not Passed, not GeneralFail, not Retransmission")},
                            }
                        }
                        Err(error) => error_handler::fail_log_handler(&format!(
                            "failed in process_configs: {:?} Exit",
                            error
                        )),
                    };
                }
                const_enum::ManageMode::Stop => break,
                _ => logger::log_debug("spawn_worker received not Echo, not Start, not Stop"),
            }
        }
        logger::log_debug("The worker has stopped!");
     
        tx_main.send(const_enum::ManageMode::Restart);
    });

    _tx
}
