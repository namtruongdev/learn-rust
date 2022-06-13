use ::regex::Regex;
use rand::Rng;
use std::fs;
use std::io;

fn reverse_string(s: String) -> String {
    // exmaple
    let mut reversed = String::new();
    for c in s.chars() {
        reversed.insert(0, c);
    }
    reversed
}

fn reverse_string_2(s: String) -> String {
    // exmaple
    s.chars().rev().collect()
}

fn lottery() {
    // example
    loop {
        let mut input = String::new();
        println!("Enter your number 0 - 99 digit lottery number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.pop();

        let input_to_int = input.parse::<i32>().expect("Please enter a number");

        if input_to_int < 0 || input_to_int > 99 {
            println!("Please enter a number between 0 and 99");
        }

        let rand_number = rand::thread_rng().gen_range(0..99);
        if input_to_int == rand_number {
            println!("You win lottery!");
        }

        println!("Goodluck next time!");
    }
}

fn homework_1() {
    let org_arr = [1, 2, 3, 5, 6, 10, 8, 11];
    let sub_arr = [6, 8, 10];
    let mut result = String::new();

    for x in org_arr.iter() {
        if sub_arr.contains(x) {
            result.push_str(&x.to_string());
        }
    }

    let sub_arr_to_string = sub_arr.iter().map(|&x| x.to_string()).collect::<String>();

    if result == sub_arr_to_string {
        println!("true");
        return;
    }

    println!("false");
}
fn homework_2() {
    let mut input = String::new();
    println!("Enter any words you want search: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.pop();

    let contents = fs::read_to_string("./src/paragraph.txt").expect("Failed to read file");

    let re = Regex::new(&format!("(?i){}", regex::escape(&input))).unwrap();

    // count all word match with regex

    let count = re.find_iter(&contents).count();

    println!("Count \"{}\" is {} words.", input, count);
}
fn main() {
    const STR_1: &str = "Trường";
    println!("Reverse string: {}", reverse_string(STR_1.to_string()));

    const STR_2: &str = "Nam";
    println!("Reverse string 2: {}", reverse_string_2(STR_2.to_string()));

    // lottery();

    homework_1();

    homework_2()
}
