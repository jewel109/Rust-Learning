fn main(){

    let s1 = String::from("hello");
    let len = calculate_size(&s1); // here s1 refferecnc is passed 
    println!("The length of {} is {}", s1, len);

    let mut s2 = String::from("Hello");
    change(&mut s2);
}

//  s is passed by refference 
fn calculate_size(s: &String) -> usize {
    s.len()

}

fn change(some_string: &mut String){
    some_string.push_str(", world")
}
