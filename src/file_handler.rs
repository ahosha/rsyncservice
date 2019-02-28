/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */


use std::fs::File;
use std::io::Read;
use chrono::prelude::*;
use logger;


pub fn read_from_file(file_name: &str)  -> Option<String> {
    logger::log_info(&format!("read from file: {}", file_name));
    let mut f = File::open(file_name).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    Some(contents)
}