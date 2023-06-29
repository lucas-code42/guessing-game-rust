use rand::Rng; // 0.8.5 add cargo.toml [dependencies]
use std::io;

fn random_number() -> i32 {
    // Generate random number in the range [0, 99]
    let num: i32 = rand::thread_rng().gen_range(0..100);
    return num;
}

fn get_user_input() -> i32 {
    let mut input: String = String::new();
    _ = io::stdin().read_line(&mut input);

    let number_input: i32 = input.trim().parse::<i32>().unwrap();
    return number_input;
}

fn main() {
    let n: i32 = random_number();

    loop {
        println!("Make a number guess between 0 and 99");
        let guess: i32 = get_user_input();

        match guess {
            _ if guess > n => println!("Try a lower number"),
            _ if guess < n => println!("Try a higher number"),
            _ if guess == n => {
                println!("You win!");
                break;
            }
            _ => println!("Invalid input. Try again."),
        }
    }
    println!("fim")
}
