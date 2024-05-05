fn longest_common_prefix(strs: Vec<String>) -> String {
 if strs.is_empty() {
     return String::new(); // If the input vector is empty, return an empty string
 }

 let mut prefix = strs[0].clone(); // Start with the first string as the prefix

 for s in strs.iter().skip(1) {
     // Iterate over the remaining strings in the vector
     while !s.starts_with(&prefix) {
         // While the current string does not start with the current prefix
         prefix.pop(); // Remove the last character from the prefix
         if prefix.is_empty() {
             // If the prefix becomes empty, there is no common prefix
             return String::new();
         }
     }
 }

 prefix // Return the longest common prefix
}

fn main() {
 let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
 println!("Longest Common Prefix: {}", longest_common_prefix(strs));

 let strs2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
 println!("Longest Common Prefix: {}", longest_common_prefix(strs2));
}
