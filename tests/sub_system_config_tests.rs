extern crate rsyncservice;
use rsyncservice::subsystem_toml_configurator::{SystemConfig,Files};

#[test]
#[should_panic]
fn subsystemconfig_new_fail() {
    let content: String = String::from("SystemConfig");
    SystemConfig::new(&content,String::from("1234"));
}

#[test]
#[should_panic]
fn subsystemconfig_new_files_pass() {
    let content: String = String::from(r#"
            config_file_path="123"

            [files]
            paths = ["/files/anchor/"]

        "#);
    let system_config = SystemConfig::new(&content,String::from("1234"));
    let test_files = Files {paths: vec![String::from("/files/anchor/")]};
    assert!(test_files ==  system_config.unwrap().files);
}

#[test]
#[should_panic]
fn subsystemconfig_two_config_pathes_pass() {
    let content: String = String::from(r#"
            config_file_path="123"
            [files]
            paths = ["/files/anchor1/", "/files/anchor2/"]
        "#);
    let system_config = SystemConfig::new(&content,String::from("1234"));
    let test_files = Files {paths: vec![String::from("/files/anchor/")]};
    assert!(2 ==  system_config.unwrap().files.paths.len());
}


