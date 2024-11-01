use std::io;

fn is_prime(n: u32) -> bool
{
    if n < 2
    {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32)
    {
        if n % i == 0
        {
            return false;
        }
    }
    true
}

fn main()
{
    println!("Введіть число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");

    let number: u32 = input.trim().parse().expect("Будь ласка, введіть коректне число");

    if is_prime(number)
    {
        println!("{} є простим числом.", number);
    }
    else
    {
        println!("{} не є простим числом.", number);
    }
}
