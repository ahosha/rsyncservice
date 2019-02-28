
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */


use std::path::Path;


use logger;


pub fn get_parent_path_string(path: &str) -> &Path {
    let path = Path::new(path);
    if path.parent().is_some() {
        path.parent().unwrap()
    } else
    {
        Path::new("")
    }
}

// pub fn is_in_service_conf_dir(path_str: &str) -> bool {
//     let mut path = environment::get_exe_path().expect(const_name::MESSAGE_GEN_ERROR);
//     path.pop();
//     path.pop();
//     path.pop();
//     let exec_dir = &path.to_str().unwrap();
//     let path_parent_dir = get_parent_path_string(&*path_str).to_str().unwrap();


//     logger::log_debug(&format!("is_local exec_dir:{} parent_dir_path:{}",
//                                exec_dir,
//                                path_parent_dir));
//     if str::eq(exec_dir,path_parent_dir) {
//         logger::log_debug(&format!("SAME PATH !!!!!!!!NEED RESTART SERVICE!!!!!! exec_dir:{} parent_dir_path:{}",
//                                    exec_dir,
//                                    path_parent_dir));
//         true
//     } else { false }
// }


// pub fn is_local(path_str: &str) -> bool {
//     let mut path = environment::get_exe_path().expect(const_name::MESSAGE_GEN_ERROR);
//     path.pop();
//     path.pop();
//     path.pop();
//     let exec_dir = &path.to_str().unwrap();
//     let path_parent_dir = get_parent_path_string(&*path_str).to_str().unwrap();
//     logger::log_debug(&format!("is_local exec_dir:{} parent_dir_path:{}",
//                                exec_dir,
//                                path_parent_dir));
//     if str::eq(exec_dir,path_parent_dir) {
//         logger::log_debug(&format!("SAME PATH !!!!!!!!NEED RESTART SERVICE!!!!!! exec_dir:{} parent_dir_path:{}",
//                                    exec_dir,
//                                    path_parent_dir));
//         true
//     } else { false }
// }

pub fn is_local_config_changes(path_str: &str, config_path: &str) -> bool {

    let path_parent_dir = get_parent_path_string(&*path_str).to_str().unwrap();
    logger::log_debug(&format!("is_local_config_changes config_path:{} parent_dir_path:{}",
                               config_path,
                               path_parent_dir));
    if str::eq(config_path,path_parent_dir) {
        logger::log_debug(&format!("SAME PATH !!!!!!!!NEED RESTART SERVICE!!!!!! exec_dir:{} parent_dir_path:{}",
                                   config_path,
                                   path_parent_dir));
        true
    } else { false }
}



