use std::convert::TryFrom;

use tch::Tensor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn func1() {
    let a = Tensor::try_from(vec![1, 2, 3]);
    println!("{:?}", a);
}
