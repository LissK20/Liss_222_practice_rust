fn main()
{
    let triangle_count = 5;
    draw_tree(triangle_count);
}

fn draw_tree(triangle_count: usize)
{
    for i in 1..=triangle_count
    {
        let height = i;

        for row in 0..height
        {
            (0..triangle_count - row).for_each(|_| print!(" "));
            (0..(2 * row + 1)).for_each(|_| print!("*"));
            println!();
        }
    }

    for _ in 0..triangle_count
    {
        (0..triangle_count).for_each(|_| print!(" "));
        println!("*");
    }
}
