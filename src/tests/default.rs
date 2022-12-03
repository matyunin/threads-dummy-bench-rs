#[cfg(test)]
mod tests {
    use crate::LRU;

    #[test]
    fn it_works() {
        let result = LRU::default();
        assert_eq!(result.count, 0);
    }
}
