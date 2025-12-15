fn main(){
let is_male = true;
let is_18 = true;
    if is_male {
        print!("Is a Male \n");
    }
    else{
        print!("Not a Male \n");
    }
    if is_male && is_18 {
        print!("male and adult \n");
    }
    let a : &str = "Hello World \n";
    let b : String = String::from("Hello World");
    print!("{}",a);
    print!("{}",b);
}
