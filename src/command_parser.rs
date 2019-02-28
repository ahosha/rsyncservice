/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

use regex::Regex;
use const_name;

//Extracts the DRBD rate from the input string which is the drbd config
fn extract_rate(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(const_name::REGEX_DRBD_RATE).unwrap();
    }
    RE.captures(input).and_then(|cap| {
        cap.name("rate").map(|rate| rate.as_str())
    })
}

///
/// Extracts the sync rate from DRBD conf file
/// 
///  
pub fn get_sync_rate (config_content: &str) -> i32  {
    let mut return_value = 0;
    if let Some(rate) = extract_rate(config_content) {
        if rate.trim().is_empty() {
                return_value = 0;

        } else {
                let rate_int: i32 = rate.parse().unwrap();
                return_value =rate_int*512;
        }
    }
    return_value
}


pub fn extract_host_version(input: &str) -> String {
    let mut result = String::new();
    let re = Regex::new(const_name::REGEX_QRADAR_VERSION).unwrap();
    for cap in re.captures_iter(input) {
        let res = format!("{}.{}.{}",&cap[1], &cap[2], &cap[3]);
        result.push_str(&res);
    }
    result
}





