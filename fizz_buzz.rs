// Rust

fn fizz_buzz(n: usize){
   for i in 1..(n+1) {
       let mut str = "".to_string();
       if i % 3 == 0 { str += "Fizz"; }
       if i % 5 == 0 { str += "Buzz"; }            
       if str.len() > 0 { println!("{}", str); }
       else { println!("{}", i); }
   }
}

fn main(){
   let num = 100;
   fizz_buzz(num);
}