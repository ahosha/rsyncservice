/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */



use std::sync::mpsc::{channel};
use const_enum;
use worker;
use logger;

//TODO: path Canonicalization ?
//TODO: add watcher factory
pub fn race() {

    let (tx_main, rx_main) = channel();
    let worker = worker::spawn_worker(tx_main.clone());

    worker.send(const_enum::ManageMode::Start).unwrap();

    loop {
        let msg = rx_main.recv().unwrap();
        match msg {
            const_enum::ManageMode::Start => println!("Start received in MAIN"),
            const_enum::ManageMode::Stop => println!("Stop received in MAIN"),
            const_enum::ManageMode::Restart => {
                println!("Restart received in MAIN will send start to worker");
                let worker_inner = worker::spawn_worker(tx_main.clone());
                worker_inner.send(const_enum::ManageMode::Start).unwrap();
            },
            _ => { logger::log_debug("Runner -> race received not Echo, not Start, not Stop, not Restart")},

        }
    }


}





