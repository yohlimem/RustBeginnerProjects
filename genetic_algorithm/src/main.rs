use std::io;
use rand::Rng;
static ASCII_ENGLISH: [char; 53] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z', 'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z', ' ',
];


fn main() {

    let mut start_string = String::new();
    const ITERATIONS:u64 = 1000;
    let mut array:Vec<u8> = Vec::new();
    io::stdin().read_line(&mut start_string).expect("couldnt read line");
    // println!("random letter: {}", rand_letters(10));
    let first_eve = String::from("a").repeat(start_string.len() - 2);
    println!("{}", start_string.len());

    let mut best_etter = first_eve;
    for _ in 0..ITERATIONS{
        let evo = evolve(&best_etter, &start_string, array.clone());
        best_etter = evo.1.clone();
        array = evo.0.clone();
        if best_etter == start_string {
            println!("hi");
            break;
        }

    }
    println!("you're word is: {}", best_etter);
    let mut not_done = String::new();
    io::stdin().read_line(&mut not_done).expect("L");
}

fn rand_letters(how_many: u8) -> String {
    let mut string = String::new();
    for _ in 0..how_many{
        string.push(ASCII_ENGLISH[rand::thread_rng().gen_range(0..ASCII_ENGLISH.len())]);
    }
    return string;
}

fn evolve(string: &String, desired_string: &String, add_array: Vec<u8>) -> (Vec<u8>, String){
    let mut new_string = string.clone();
    let mut new_array = add_array.clone();
    // let mut string_check = desired_string.clone();
    for _ in 0..new_string.len(){
            for y in 0..new_string.len(){
                // let y:u8 = y as u8;
                if new_string[y..y+1] != desired_string[y..y+1] { // flow; this will check for the the last string on not for the string below.
                    let replacement = &rand_letters(1);
                    new_string.replace_range(y..y + 1, replacement);
                }
                else {
                    if !new_array.contains(&(y as u8)){
                        new_array.push(y as u8);
                    }
                 }
            }

    }
    println!("the string evolving: {new_string}");
    return (new_array.clone(), new_string.clone());
}