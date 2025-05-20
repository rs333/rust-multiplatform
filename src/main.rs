fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use helpers::SegmentTree;

    #[test]
    fn hello() {
        let segment = SegmentTree::new(vec![4, 3, 7, 8, 9, 4, 14, 9, 3, 15, 16]);
        println!("{}", segment);
    }
}
