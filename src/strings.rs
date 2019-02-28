
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */

use logger;
use environment;

// pub fn combine_path(common_path: &str, paths: &str) ->  Vec<String> {

//     let paths: Vec<&str> = paths.split(':').collect();
//     let mut vec_to_ret = Vec::new();
//     //concat path
//     //TODO:  rewrite it with .iter  and .map for init vector
//     //(1..100).map(|x| logger::log_info(&format!("{}", x));
//     //https://doc.rust-lang.org/1.8.0/book/iterators.html
//     let path_it = paths.into_iter();
//     for x in path_it {
//         if !x.trim().is_empty() && is_legal_path(x)
//             {
//                 let combination = format!("{}{}", common_path, x);
//                 vec_to_ret.push(combination);
//             }
//     }
//     vec_to_ret
// }

pub fn is_legal_path (path: &str) -> bool {
    let mut result = true;
    if path.len() > 3 {
        let (_path1, illigal) = path.split_at(path.len() - 3);
        if illigal == "___" { result = false; }
    }
    result
}

pub fn verify_input_path(paths: Vec<String>) ->  Vec<String> {
    let mut vec_to_ret = Vec::new();
    for x in paths.iter() {
        if !x.trim().is_empty()
            {
                let paths_collection: Vec<&str> = x.split(',').collect();
                for path in paths_collection.iter() {
                    if !path.trim().is_empty() && is_legal_path(path) {
                            logger::log_debug(&format!("verify_input_path {}", &path));
                            if !environment::path_exists(&path) {
                                logger::log_debug(&format!("path:{} doesn't exists. won't be monitored", &path));
                            } else {
                                logger::log_debug(&format!("path:{} exists.", &path));
                                vec_to_ret.push(path.to_string());
                            }
                        }
                    }
            }
    }
    logger::log_debug(&format!("verify_input_path return collection of {} elements", vec_to_ret.len()));
    vec_to_ret
}

