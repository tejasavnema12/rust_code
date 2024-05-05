fn find_median(arr: &[i32]) -> f64 {
 let n = arr.len();

 if n % 2 == 0 {
     // If the array length is even, return the average of the middle two elements
     let middle_index1 = (n - 1) / 2;
     let middle_index2 = middle_index1 + 1;
     let median = (arr[middle_index1] as f64 + arr[middle_index2] as f64) / 2.0;
     median
 } else {
     // If the array length is odd, return the middle element
     let middle_index = n / 2;
     arr[middle_index] as f64
 }
}

fn main() {
 let arr = [1, 2, 9, 4, 5];
 println!("Median: {}", find_median(&arr));

 let arr2 = [1, 2, 3, 4, 5, 6];
 println!("Median: {}", find_median(&arr2));
}
