
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */


use const_name;
use pre_checkers_common;

pub struct QradarStatusChecker {
    pub remote_ip: String
}

pub struct HASetupStatusChecker ();

pub struct HARemoteStateChecker ();

pub struct HostVersionChecker ();

pub struct QradarVersionChecker ();


//TODO: refactor get_qradar_status function
impl pre_checkers_common::Checker for QradarStatusChecker {
    fn check(&self)  -> bool {
        let qradar_get_status = &(String::from(const_name::QRADAR_ENVIRONMENT)+ "bin/getstatus.sh");
        pre_checkers_common::check_qradar_status(qradar_get_status,
                                                &self.remote_ip)
    }
}

impl pre_checkers_common::Checker for HASetupStatusChecker {
    fn check(&self) -> bool {
        pre_checkers_common::check_ha_setup_is_running()
    }
}

impl pre_checkers_common::Checker for HARemoteStateChecker {
    fn check(&self) -> bool {
        let skip_token = &(String::from(const_name::QRADAR_ENVIRONMENT)+ "ha/.ha_skip_remote_state_rsync_check");
        pre_checkers_common::get_ha_remote_state(const_name::HA_SCRIPT,
                                                 &skip_token)
    }
}


impl pre_checkers_common::Checker for HostVersionChecker {
    fn check(&self) -> bool {
        let host_version_script=&(String::from(const_name::QRADAR_ENVIRONMENT)+ "bin/getHostVersion.sh");
        pre_checkers_common::check_host_version(host_version_script)
    }
}

impl pre_checkers_common::Checker for QradarVersionChecker {
    fn check(&self) -> bool {
        let build_number=&(String::from(const_name::QRADAR_ENVIRONMENT)+ "bin/getHostVersion.sh");
        pre_checkers_common::check_qradar_build_number(build_number)
    }
}






