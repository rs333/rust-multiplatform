struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut n = nums1.len() + nums2.len();
        // 3/2 1
        // 2/2 1
        let mut m1 = nums1.len() / 2;
        let mut m2 = nums2.len() / 2;
        let right1 = nums1.len() - m1;
        let left1 = nums1.len() - right1;
        let right2 = nums2.len() - m2;
        let left2 = nums2.len() - right2;
        let si = nums1.len() - 1;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn example1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let answer = super::Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(2., answer);
    }
    fn example2() {
        let nums1 = vec![1, 3];
        let nums2 = vec![3, 4];
        let answer = super::Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(2., answer);
    }
}
