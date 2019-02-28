 use std::sync::mpsc::channel;
 use std::thread;
 use std::time::Duration;

 use logger;


 // pub fn start_full_sync(sync_period: u64) {
 //     let (full_tx, full_rx) = channel();
 //     logger::log_debug(&format!("start full sync cycle with sync period {} sec ", sync_period));

 //     thread::spawn(move || {
 //         loop {
 //             thread::sleep(Duration::from_secs(sync_period));
 //             full_tx.send("start").unwrap();
 //         }
 //     });

 //     loop {
 //         let _ = full_rx.try_recv().map(|reply| init_full_sync(reply));
 //     }
 // }

 fn init_full_sync (params: &str)
 {
     logger::log_debug(&format!("init_full_sync {}", params));
 }
