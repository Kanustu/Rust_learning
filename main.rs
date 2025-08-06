fn main (){
// assigning variables and modulo of variables
let var1 = 5;
let var2 = 2;
let ans = var1 % var2;
println!("{}", ans);

// creating vector and removing and adding a value
let mut vec1 = vec![2, 4, 6, 8, 10];
println!("{:?}", vec1);
vec1.pop();
vec1.push(12);
println!("{:?}", vec1);

//concat "Hello World"
let str1 = String::from("Hello ");
let ans = concat_string(str1);
println!("{}", ans);


control_flow(1);
control_flow(4);
control_flow(125);
control_flow(32);

}

fn concat_string (val: String) -> String {
    val + "World!"
}

fn control_flow (val: i32){
    if val == 1 {
        println!("The value is one");
    } else if val < 25 {
        println!("The value is less than 25");
    } else if val > 50 {
        println!("The vale is greater than 50");
    } else {
        println!("The value is greater than 25 but less than 50");
    }
}

