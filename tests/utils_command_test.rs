extern crate rsyncservice;
use rsyncservice::command_parser;


#[test]
fn get_sync_rate_passed() {
    let config_content = String::from("resource store {
	disk {
	    resync-rate 100M;al-extents 1237;
		c-max-rate 100M;c-plan-ahead 20;c-fill-target 0;c-delay-target 1;c-min-rate 4096;
	    disk-barrier no;
	    disk-flushes no;
	    md-flushes no;
	    on-io-error detach;
	} } ");
    let rate = command_parser::get_sync_rate(&config_content);
    assert_eq!(rate, 51200);
}

#[test]
fn get_sync_rate_passed_zero() {
    let config_content = String::from("resource store {
	disk {
	    resync-rate 0M;al-extents 1237;
		c-max-rate 100M;c-plan-ahead 20;c-fill-target 0;c-delay-target 1;c-min-rate 4096;
	    disk-barrier no;
	    disk-flushes no;
	    md-flushes no;
	    on-io-error detach;
	} } ");
    let rate = command_parser::get_sync_rate(&config_content);
    assert_eq!(rate, 0);
}


#[test]
fn get_sync_rate_passed_empty() {
    let config_content = String::from("resource store {
	disk {
		c-max-rate 100M;c-plan-ahead 20;c-fill-target 0;c-delay-target 1;c-min-rate 4096;
	    disk-barrier no;
	    disk-flushes no;
	    md-flushes no;
	    on-io-error detach;
	} } ");
    let rate = command_parser::get_sync_rate(&config_content);
    assert_eq!(rate, 0);
}


#[test]
fn get_host_version_empty() {
    let host_version_content = String::from("isConsole=\"true\"
                                                applianceType=\"3199\"
                                                qradarVersion=7.3.2
                                                hardwareSerial=\"VMware-42 26 a3 70 36 b0 99 28-b6 e0 9b 09 f1 e6 ac 67\"");
    let host_version = command_parser::extract_host_version(&host_version_content);
    assert_eq!(host_version,String::from("7.3.2"));
}
