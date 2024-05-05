fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
 if k <= 0 || k > arr.len() {
     return None; // If k is out of bounds, return None
 }

 // Sort the array to find the kth smallest element
 let mut sorted_arr = arr.to_vec();
 sorted_arr.sort();

 Some(sorted_arr[k - 1]) // Return the kth smallest element
}

fn main() {
 let arr = [7, 10, 4, 3, 20, 15];
 let k = 3;
 match kth_smallest(&arr, k) {
     Some(kth) => println!("The {}th smallest element is: {}", k, kth),
     None => println!("Invalid input or k is out of bounds"),
 }
}
