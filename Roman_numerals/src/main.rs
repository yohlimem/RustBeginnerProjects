use std::io;

fn main() {
    let mut roman_numerals = String::new();
    std::io::stdin().read_line(&mut roman_numerals).expect("couldn't read line");
    convert_roman_numerals(roman_numerals.to_ascii_uppercase());
}

fn convert_roman_numerals(numerals: String){

    let mut num = 0;

    for i in 0..numerals.len(){
        if(i >= 1){

            if numerals[i..i+1] == String::from("I"){
                num += 1;
            } else if numerals[i-1..i+1] == String::from("IV"){
                num += 4;
            } else if numerals[i..i+1] == String::from("V") && numerals[i-1..i+1] != String::from("IV"){
                num += 5;
            } else if numerals[i-1..i+1] == String::from("IX"){
                num += 9;

            } else if numerals[i..i+1] == String::from("X") && numerals[i-1..i+1] != String::from("IX"){
                num += 10;

            } else if numerals[i-1..i+1] == String::from("XL"){
                num += 40;

            } else if numerals[i..i+1] == String::from("L") && numerals[i-1..i+1] != String::from("XL"){
                num += 50;

            } else if numerals[i-1..i+1] == String::from("XC"){
                num += 90;

            } else if numerals[i..i+1] == String::from("C"){
                num += 100;

            } else if numerals[i-1..i+1] == String::from("CD"){
                num += 400;

            } else if numerals[i..i+1] == String::from("D"){
                num += 500;

            } else if numerals[i-1..i+1] == String::from("CM"){
                num += 900;

            } else if numerals[i..i+1] == String::from("M"){
                num += 1000;

            }
        }
    }
    println!("number: {}", num);
    std::io::stdin().read_line(&mut String::new()).expect("couldn't read line");

}
