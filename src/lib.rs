pub mod atproto;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _did = crate::atproto::Did {
            method: "".to_string(),
            identifier: "".to_string(),
        };
        
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
