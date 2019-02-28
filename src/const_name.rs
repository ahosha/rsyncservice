/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

///
/// The file has all the constants being used in the rsync service code
/// 

// Environment specific constants
pub static ENV_VAR_NMAE_RSYNC_CONFIG_FILE : &str =  "rsync_config_file";
pub static QRADAR_ENVIRONMENT : &str = "/opt/qradar/";
pub static HA_SCRIPT : &str =  "/opt/qradar/ha/bin/ha";

// Commands specific constants
pub static DRBD_CONF_FILE_NAME : &str =  "/etc/drbd.conf";
pub static HA_SETUP_SCRIPT : &str =  "ha_setup.sh";
pub static PID_SCRIPT : &str =  "pidof";
pub static PID_SCRIPT_EXISTS_ARG : &str =  "-x";
pub static HA_SCRIPT_REMOTE_ARG : &str =  "remote_state";

// Mock code specific constants
pub static MOCK_SCRIPT_ROOT_FOLDER : &str =  "qradar_mock_scripts";
pub static MOCK_SCRIPT_GET_STATE : &str =  "getStatus.sh";
pub static MOCK_HA_SCRIPT : &str =  "ha";
pub static MOCK_SCRIPT_SKIP_REMOTE_STATE : &str =  "ha_skip_remote_state_rsync_check";
pub static MOCK_SCRIPT_GET_QRADAR_BUILD : &str =  "myver_version.sh";
pub static MOCK_SCRIPT_GET_HOST_VERSION : &str =  "getHostVersion.sh";

// General constants related to messages and functions for rsync service
pub static MESSAGE_GEN_ERROR : &str =  "error";
pub static MESSAGE_NEED_UPDATE : &str =  "STATUS=needs_upgrade";
pub static REGEX_QRADAR_VERSION : &str = r"qradarVersion=(\d{1}).(\d{1}).(\d{1})";
pub static REGEX_DRBD_RATE : &str = r"resync-rate (?P<rate>\d[0-9]+)[0-9a-zA-Z_];";







