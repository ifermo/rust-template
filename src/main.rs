fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
        let a = 1;
        let b = 2;
        assert_eq!(a + b, 3);
    }
}
