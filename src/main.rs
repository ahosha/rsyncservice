/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

extern crate  rsyncservice;
use rsyncservice::run;
use std::thread;


//todo : add return error::Result<()>
///run with following set of environtment variable
/// export RUST_BACKTRACE=1 - allow to receive backtrace
/// or run ``` RUST_BACKTRACE=1 cargo run ```
/// Could set it to 'auto' as default, which would only show the backtrace if 'Debug' not when 'Stable'


pub fn main() {

    let child = thread::spawn(move || {
        run::race();    });
    let _res = child.join();


}



//todo:
// unittests
// logger factory
// pre-post-script run support
// for aradar integration :
//  1. add merge_conf_files()
//  2. get_local_state
//  3. get exclude

