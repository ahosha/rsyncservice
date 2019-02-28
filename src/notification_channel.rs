/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */


use std::cell::RefCell;
use logger;




pub struct NotificationChannel{
    pub vec: RefCell<Vec<String>>,

}

impl NotificationChannel {
    pub fn created()->NotificationChannel {
        NotificationChannel {
            vec: RefCell::new(Vec::new()),
        }
    }
    pub fn add(&self, value: String){
        self.vec.borrow_mut().push(value);
        logger::log_debug(&format!("Notification_Channel add path for sync" ));
    }
    pub fn clear(&self){
        self.vec.borrow_mut().clear();
        logger::log_debug(&format!("Notification_Channel clear inner queue" ));
    }
    
    pub fn get_inner_vector(&self) -> Vec<String>{
        
        let ref_vec = self.vec.borrow();
        ref_vec.clone()
    }
}
