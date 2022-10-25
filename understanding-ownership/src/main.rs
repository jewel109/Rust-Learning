fn main(){

    let word = first_word(&s);

    println!("{}",word);

    s.clear();

    println!("the first word is : {}",word);

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
