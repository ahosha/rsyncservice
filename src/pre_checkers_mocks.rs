
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */


///
/// Prechecker implementation for mock scripts which involve some common checks used in actual Qradar prechecks
/// 
use environment;
use const_name;
use pre_checkers_common;

pub struct QradarStatusCheckerMock ();

pub struct HASetupStatusCheckerMock {
    pub check_passed: bool
}

pub struct HARemoteStateCheckerMock ();

//pub struct HostRemoteVersionCheckerMock ();

pub struct HostVersionCheckerMock ();

//pub struct QradarRemoteVersionCheckerMock ();

pub struct QradarVersionCheckerMock ();

//TODO: refactor get_qradar_status function
impl pre_checkers_common::Checker for QradarStatusCheckerMock {
    fn check(&self)  -> bool {
        let path = environment::get_mock_file_path(const_name::MOCK_SCRIPT_GET_STATE)
                            .expect(const_name::MESSAGE_GEN_ERROR);
        let cmd = String::from(path.to_str().unwrap());
        pre_checkers_common::check_qradar_status(&cmd,&String::new())
    }
}

impl pre_checkers_common::Checker for HASetupStatusCheckerMock {
    fn check(&self) -> bool {
        if self.check_passed {
            true
        }  else {
            false
        }
    }
}

impl pre_checkers_common::Checker for HARemoteStateCheckerMock {
    fn check(&self) -> bool {
        let path = environment::get_mock_file_path(const_name::MOCK_HA_SCRIPT)
                            .expect(const_name::MESSAGE_GEN_ERROR);
        let skip_path = environment::get_mock_file_path(const_name::MOCK_SCRIPT_SKIP_REMOTE_STATE)
            .expect(const_name::MESSAGE_GEN_ERROR);
        pre_checkers_common::get_ha_remote_state(&String::from(path.to_str().unwrap()),
                                                 &String::from(skip_path.to_str().unwrap()))
    }
}

impl pre_checkers_common::Checker for HostVersionCheckerMock {
    fn check(&self) -> bool {
        let path = environment::get_mock_file_path(const_name::MOCK_SCRIPT_GET_HOST_VERSION)
            .expect(const_name::MESSAGE_GEN_ERROR);
        pre_checkers_common::check_host_version(&String::from(path.to_str().unwrap()))
    }
}

impl pre_checkers_common::Checker for QradarVersionCheckerMock {
    fn check(&self) -> bool {
        let path = environment::get_mock_file_path(const_name::MOCK_SCRIPT_GET_QRADAR_BUILD)
            .expect(const_name::MESSAGE_GEN_ERROR);
        pre_checkers_common::check_qradar_build_number(&String::from(path.to_str().unwrap()))
    }
}



