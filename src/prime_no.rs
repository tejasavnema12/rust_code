fn is_prime(num: u64) -> bool {
 if num <= 1 {
     return false; // 0 and 1 are not prime numbers
 }
 if num == 2 {
     return true; // 2 is a prime number
 }
 if num % 2 == 0 {
     return false; // Even numbers (except 2) are not prime
 }

 let mut divisor = 3;
 while (divisor * divisor) <= num {
     if num % divisor == 0 {
         return false; // If num is divisible by any divisor, it's not prime
     }
     divisor += 2; // Skip even divisors
 }

 true // If no divisor divides num, it's prime
}

fn main() {
 let num = 23;
 if is_prime(num) {
     println!("{} is a prime number", num);
 } else {
     println!("{} is not a prime number", num);
 }
}
