use std::io;

fn is_palindrome(n: i32) -> bool
{
    let original = n.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

fn main()
{
    println!("Введіть число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");

    let number: i32 = input.trim().parse().expect("Будь ласка, введіть коректне число");

    if is_palindrome(number)
    {
        println!("{} є паліндромом.", number);
    }
    else
    {
        println!("{} не є паліндромом.", number);
    }
}
