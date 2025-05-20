use std::fmt::Display;
pub struct SegmentTree {
    tree: Vec<i32>,
    n: usize,
}
impl Display for SegmentTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = 2;
        let spacing = 3;
        let mut first: usize = 1;
        let mut last = first;
        let total_width = self.n * (width + spacing) - spacing;
        while last < self.tree.len() {
            let count = last - first + 1;
            let current_spacing = (total_width + spacing) / count - width;
            let left_spacing = current_spacing / 2;
            let right_spacing = current_spacing - left_spacing;
            for i in first..=last {
                write!(f, "{:<1$}", "", left_spacing)?;
                write!(f, "{:<1$}", self.tree[i], width)?;
                write!(f, "{:<1$}", "", right_spacing)?;
            }
            writeln!(f)?;
            first *= 2;
            last = last * 2 + 1;
        }
        Ok(())
    }
}

impl SegmentTree {
    pub fn new(vals: Vec<i32>) -> Self {
        let n = vals.len().next_power_of_two();
        let mut tree = vec![0; 2 * n];

        tree[n..(n + vals.len())].copy_from_slice(&vals);

        for i in (1..n).rev() {
            tree[i] = tree[2 * i] + tree[2 * i + 1];
        }
        Self { tree, n }
    }

    pub fn sum(&self, mut l: usize, mut r: usize) -> i32 {
        let mut s = 0;
        l += self.n;
        r += self.n;

        while l <= r {
            if l % 2 == 1 {
                s += self.tree[l];
                l += 1;
            }
            if r % 2 == 0 {
                s += self.tree[r];
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }

        s
    }

    pub fn add(&mut self, mut k: usize, x: i32) {
        k += self.n;
        self.tree[k] += x;
        k /= 2;
        while k >= 1 {
            self.tree[k] = self.tree[2 * k] + self.tree[2 * k + 1];

            k /= 2;
        }
    }

    pub fn print(&self) {
        for val in &self.tree {
            print!("{val} ");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTree;

    #[test]
    fn create_power_of_2() {
        let invals = vec![5, 8, 6, 3, 2, 7, 2, 6];
        let segment = SegmentTree::new(invals);
        segment.print();
        assert_eq!(26, segment.sum(2, 7));
    }
    #[test]
    fn create_non_power_of_2() {
        let invals = vec![5, 8, 6, 3, 2];
        let segment = SegmentTree::new(invals);
        segment.print();
        assert_eq!(5, segment.sum(0, 0));
        assert_eq!(5, segment.sum(3, 4));
    }
    #[test]
    fn add() {
        let mut segment = SegmentTree::new(vec![5, 8, 6, 3, 2]);
        assert_eq!(5, segment.sum(0, 0));
        assert_eq!(13, segment.sum(0, 1));
        assert_eq!(5, segment.sum(3, 4));
        segment.add(0, 4);
        assert_eq!(9, segment.sum(0, 0));
        assert_eq!(17, segment.sum(0, 1));
        assert_eq!(5, segment.sum(3, 4));
    }
}
