// Define a function called is_palindrome that takes a string slice (&str) as input
fn is_palindrome(s: &str) -> bool {
 // Create a new string called reversed by iterating over each character in the input string in reverse order
 let reversed: String = s.chars().rev().collect();
 // Check if the original string is equal to the reversed string
 s == reversed
}

// Define the main function
fn main() {
 // Define test cases as an array of string literals
 let test_cases = vec!["radar", "hello", "level", "12321"];

 // Iterate over each test case
 for case in test_cases {
     // Check if the current test case is a palindrome using the is_palindrome function
     if is_palindrome(case) {
         // If the test case is a palindrome, print that it is a palindrome
         println!("{} is a palindrome", case);
     } else {
         // If the test case is not a palindrome, print that it is not a palindrome
         println!("{} is not a palindrome", case);
     }
 }
}
