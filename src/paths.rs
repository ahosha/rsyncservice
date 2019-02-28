//use std::cell::RefCell;

// pub struct Paths{
//     pub vec: RefCell<Vec<String>>,
// }

// impl Paths {
//     pub fn created()->Paths {
//         Paths {vec: RefCell::new(Vec::new()) }
//     }
//     pub fn add(&self, value: String){
//         self.vec.borrow_mut().push(value);
//     }
//     pub fn extend(&self, value: Vec<String>){
//         self.vec.borrow_mut().extend(value.into_iter());
//     }
//     pub fn get_inner_vector(&self) -> Vec<String>{
//         let ref_vec = self.vec.borrow();
//         ref_vec.clone()
//     }
// }
