fn main(){
if_else();
if_else_1();
string_one();
string_two();
loop_1();
tuple();
array();
int();
height(10);
car_details("BMW", 2025, "Black", true, 325.5);

}

fn height(height : i8){
    println!("Height is : {}",height)
}



fn car_details(name : &str, model:i32, color:&str, availability:bool, top_speed:f32){
print!(
    "Name : {} , Model : {}  , Color : {} , Is_Available : {} , Top_Speed : {}",
    name, model, color, availability, top_speed 
)
}



// 
fn if_else(){

let is_male = true;
let is_18 = true;
let ab:bool= false;
println!("{}",ab);
   if is_male {
       print!("Is a Male \n");
   }
   else{
       print!("Not a Male \n");
   }
   if is_male && is_18 {
       print!("male and adult \n");
   }
   let a : &str = "Hello World";
   let b : String = String::from("Hello World");
   println!("{}",a);
   print!("{}",b);
}


fn string_one(){
   let a : String = String::from("Hello World");
   println!("{}",a);
   let b = a.chars().nth(1);
   match b {
       Some(c) => println!("{}",c),
       None => println!("not found !"),
   }
}



fn if_else_1(){
   let a :i8 = -10;
   if a > 0 {
       println!("+")
   } else if a < 0 {
       println!("-")
   }
   else {
       println!("0")
   };
}



fn loop_1(){
   for i in 0..10{
       println!("{}",i);
   }
}


fn int() {
   let num :[i32;6] = [1,2,3,4,5,6];
   println!("{:?}",num);
   println!("{}",num[0]);
   println!("{}",num[1]);

}


fn tuple(){
   let num = ("Huzaif","Mushtaq",3);
   print!("{:?}",num);
}

fn array(){
   let num : &[i8] = &[1,2,3,4,5];
   println!("{:?}", num);
}


fn string_two(){
   let mut a :String = String::from("Hello");
   a.push_str(" World");
   println!("{}",a);
}