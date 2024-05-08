pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Test2 {

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
