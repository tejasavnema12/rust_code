fn is_prime(num: u64) -> bool {
 // Check if the number is less than 2, which is not prime
 if num < 2 {
     return false;
 }

 // Check if the number is 2 or 3, which are prime
 if num == 2 || num == 3 {
     return true;
 }

 // Check if the number is divisible by 2 or 3, if yes, it's not prime
 if num % 2 == 0 || num % 3 == 0 {
     return false;
 }

 // Iterate from 5 to the square root of the number in steps of 6
 // We only need to check divisors up to the square root of the number
 let mut i = 5;
 while i * i <= num {
     // Check if the number is divisible by i or i + 2, if yes, it's not prime
     if num % i == 0 || num % (i + 2) == 0 {
         return false;
     }
     // Increment i by 6 to skip even numbers and numbers divisible by 3
     i += 6;
 }

 // If the number passes all the checks, it's prime
 true
}

fn main() {
 let num = 17;
 if is_prime(num) {
     println!("{} is a prime number", num);
 } else {
     println!("{} is not a prime number", num);
 }
}
