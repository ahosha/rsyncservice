/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */
use chrono::prelude::*;
use log::*;


pub fn log_info(message: &str) {
    let mut timestmp: DateTime<Local> = Local::now();
    let info: &str="INFO";

    println!("{} [{}] {}",timestmp.to_rfc2822(),info,message); }


//pub fn log_warn(message: &str) { println!("{}",message); }

pub fn log_debug(message: &str) {
    let mut timestmp: DateTime<Local> = Local::now();
    let debug: &str="DEBUG";
    println!("{} [{}] {}",timestmp.to_rfc2822(),debug,message);
}

pub fn log_error(message: &str) {
    let mut timestmp: DateTime<Local> = Local::now();
    let error: &str="ERROR";
    println!("{} [{}] {}",timestmp.to_rfc2822(),error,message);
}




