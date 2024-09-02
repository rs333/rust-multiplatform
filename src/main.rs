fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello() {
        assert_eq!(1,1)
    }
}
