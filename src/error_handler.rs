/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

use std::process;
use logger;

pub fn warn_log_handler(message: &str) {
    logger::log_error(&message.to_string());
}

pub fn fail_log_handler(message: &str) {
    logger::log_error(&message.to_string());
    process::exit(0x0100);
}




