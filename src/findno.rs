fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
 let mut left = 0;
 let mut right = arr.len() - 1;
 let mut result = None;

 while left <= right {
     let mid = left + (right - left) / 2;
     if arr[mid] == target {
         // Store the index of the target and continue searching on the left side
         result = Some(mid);
         right = mid - 1;
     } else if arr[mid] < target {
         left = mid + 1;
     } else {
         right = mid - 1;
     }
 }

 result
}

fn main() {
 let arr = [1, 2, 2, 3, 3, 3, 4, 5, 5, 6];
 let target = 5;
 match find_first_occurrence(&arr, target) {
     Some(index) => println!("The first occurrence of {} is at index {}", target, index),
     None => println!("{} is not found in the array", target),
 }
}
