// fn main(){
//   println!("I love Coding in Rust");
// }

// fn main(){
//   let x: u32 = 5;
//   println!("Your number is {x}");
// }

// fn main(){
//   let y: u64 = 1000;
//   println!("Your number is {y}");
// }



// // fn main(){
// fn main(){
//   age();
//   birthday();
// }
// fn age(){
//   let age:i32 = 20;
//   if age == 19 {
//     println!("You are 19");
//   }
//   else {
//     println!("You are not 19");
//   }
  
// }


// //Wish a happy birthday
// fn birthday(){
//   let name = "John";
//   let age :i64 = 20;
//   println!("Happy birthday {name} you are {age} years old");
// }

fn main(){
pi();
rec();
bools();
char();
}

fn pi(){
  let _pi: f64 = 3.14;
  let radius: f64 = 5.0;
  let area: f64 = {_pi} * (radius * radius);
  println!("Area of your circle is {area}");
}

fn rec(){
  let length: f64 = 10.0;
  let width: f64 = 23.4;
  let area: f64 = length*width;
  println!("Area of your rectangle is {area}");
}
fn bools(){
  //Booleans
let is_sunny: bool = true;
let is_raining: bool = false;
println!("It is raining {}",is_raining);
println!("It is sunny {}",is_sunny);

}

fn char(){
  let letter:char = 'N';
  println!("The letter is {}",letter);
}
