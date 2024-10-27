fn main()
{
    const WIDTH: usize = 30;
    const HEIGHT: usize = 14;

    for i in 0..HEIGHT
    {
        for j in 0..WIDTH
        {
            if i == 0 || i == HEIGHT - 1
            {
                print!("*"); // Верхня та нижня межі
            }
            else if j == i || j == WIDTH - i - 1
            {
                print!("*"); // Діагональні лінії всередині
            }
            else if j == 0 || j == WIDTH - 1
            {
                print!("*"); // Бічні межі
            }
            else
            {
                print!(" "); // Пробіли всередині
            }
        }
        println!();
    }
}
