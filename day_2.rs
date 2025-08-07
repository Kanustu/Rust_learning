// vector mainpulation and boolean checks
fn main () {
    let mut var = vec![1,3,5,7];
    let result = vec_prac(var.clone()); // using clone to avoid ownership issues
    println!("{}", result);
    var.push(15);
    println!("{:?}", var);

    let result = add_two(123);
    println!("{}", result)
}

fn vec_prac(val: Vec<i32>) -> bool {
    if val[0] == 1 {
        true
    } else {
        false
    }
}

fn add_two(val: i8) -> i8 {
    val + 2
}