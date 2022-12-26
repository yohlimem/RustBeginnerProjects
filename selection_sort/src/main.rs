use std::vec::Vec;

fn main() {
    let mut how_many_string = String::new();

    std::io::stdin().read_line(&mut how_many_string).expect("among us1");

    let how_many: u32 = match how_many_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            return;
        }
    };



    let mut vector_num_array: Vec<i32> = random_vec(how_many);
    vector_num_array = sort(vector_num_array.clone());
    println!("\n you're sorted array is: \n\t{:?}", vector_num_array);
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).expect("among us1");
}

fn swap(array: Vec<i32>,element1: usize, element2: usize) -> Vec<i32>{

    let mut new_array = array.clone();

    let el1 = new_array[element1];
    let el2 = new_array[element2];
    new_array[element1.clone()] = el2;
    new_array[element2.clone()] = el1;

    return new_array.clone();
}

fn sort(array: Vec<i32>) -> Vec<i32>{

    let mut times = 0;
    let mut new_array = array;

    'L: loop {
        for i in 0..new_array.len() {
            if i > 0 {
                if new_array[i-1] > new_array[i] {
                    times = 0;
                    new_array = swap(new_array.clone(), i-1, i);
                    println!("new array: {:?}", new_array);

                } else{
                    times += 1;
                }
                if times >= new_array.len() {
                    break 'L;
                }

            }
        }

    }
    return new_array;
}

extern crate rand;

use rand::Rng;

fn random_vec(length: u32) -> Vec<i32>{
    let mut rng = rand::thread_rng();
    let mut random_numbers = Vec::new();

    for _ in 0..length {
        let random_number = rng.gen_range(-500..501);
        random_numbers.push(random_number);
    }

    return random_numbers;
}