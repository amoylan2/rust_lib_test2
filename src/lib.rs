use lazy_static::lazy_static;
use std::sync::Mutex;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Test2 {

}




lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

pub fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

pub fn get_call_count() -> usize {
    ARRAY.lock().unwrap().len()
}

// pub unsafe extern "C" fn tt() -> () {
//     println!("hello from test2")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
