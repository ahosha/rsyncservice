extern crate rsyncservice;
 use rsyncservice::service_toml_configurator::*;


#[test]
//#[should_panic]
fn serviceconfig_new_fail() {
    let content: String = String::from("String");
    ServiceConfig::new(&content);
}


#[test]
fn serviceconfig_new_destination_pass() {
    let content: String = String::from(r#"
        [destination]
            ip = "11.22.133.44"
        [config]
            config_folder = "/anchor/conf.d/"
            qradar_integrated = false
        "#);
    let service_config = ServiceConfig::new(&content);
    let test_dest = Destination {ip: String::from(r#"11.22.133.44"#)};
    assert!(test_dest == service_config.unwrap().destination);
}




#[test]
fn serviceconfig_new_config_pass() {
    let content: String = String::from(r#"
        [destination]
            ip = "11.22.133.44"
        [config]
            config_folder = "/anchor/conf.d/"
            qradar_integrated = false
        "#);
    let service_config = ServiceConfig::new(&content);
    let test_config = Config {config_folder: String::from(r#"/anchor/conf.d/"#), qradar_integrated: false};
    assert!(test_config == service_config.unwrap().config);
}



