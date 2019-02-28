
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */



///
/// Initialization of prechecker script to run prechecks on mock scripts or on actual Qradar(Ha) 
/// 
use pre_checkers_common;
use pre_checkers;
use pre_checkers_mocks;



pub fn run_checkers(qradar_integrated: bool, remote_ip: String) -> bool {
    let mut result = true;
//Running prechecks on Actual qradar
    if qradar_integrated {
        let qradar_status = pre_checkers::QradarStatusChecker{remote_ip};
        let ha_setup_status = pre_checkers::HASetupStatusChecker();
        let ha_remote_state = pre_checkers::HARemoteStateChecker();
        let host_version_checker = pre_checkers::HostVersionChecker();
        let qradar_build_checker = pre_checkers::QradarVersionChecker();

        let pre_checkers: Vec<&pre_checkers_common::Checker>  = vec![&qradar_build_checker,
                                                                     &host_version_checker,
                                                                     &qradar_status,
                                                                     &ha_setup_status,
                                                                     &ha_remote_state];

        for pre_checker in &pre_checkers {
            result = pre_checker.check();
        }

    }
    //Running prechecks on mock scripts
     else {
        let qradar_status = pre_checkers_mocks::QradarStatusCheckerMock();
        let ha_setup_status = pre_checkers_mocks::HASetupStatusCheckerMock{check_passed: true};
        let ha_remote_state = pre_checkers_mocks::HARemoteStateCheckerMock();
        let host_version_checker = pre_checkers_mocks::HostVersionCheckerMock();
        let qradar_build_checker = pre_checkers_mocks::QradarVersionCheckerMock();

        let pre_checkers: Vec<&pre_checkers_common::Checker>  = vec![&qradar_build_checker,
                                                                     &host_version_checker,
                                                                     &qradar_status,
                                                                     &ha_setup_status,
                                                                     &ha_remote_state];

        for pre_checker in &pre_checkers {
            result = pre_checker.check();
        }

    }
    result
}

