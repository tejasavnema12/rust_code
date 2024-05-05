fn max_subarray_sum(nums: &[i32]) -> i32 {
 let mut max_sum = i32::MIN; // Initialize max_sum as negative infinity
 let mut current_sum = 0;

 for &num in nums {
     current_sum = current_sum.max(0) + num; // Update current_sum to be the maximum of either itself or zero plus the current number
     max_sum = max_sum.max(current_sum); // Update max_sum to be the maximum of either itself or current_sum
 }

 max_sum
}

fn main() {
 let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4]; // Sample array
 let max_sum = max_subarray_sum(&nums);
 println!("Maximum subarray sum: {}", max_sum);
}
