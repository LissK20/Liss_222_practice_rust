fn invert_the_case(s: String) -> String
{
    s.chars()
        .map(|c|
            {
            if c.is_uppercase()
            {
                c.to_lowercase().to_string()
            }
            else
            {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

fn main()
{
    let data = [
        ("Hello", "hELLO"),
        ("Привіт", "пРИВІТ"),
    ];

    for (a, b) in data.iter()
    {
        assert_eq!(invert_the_case(a.to_string()), b.to_string());
        assert_eq!(invert_the_case(b.to_string()), a.to_string());
    }

    let input = "Hello".to_string();
    let result = invert_the_case(input);
    println!("{}", result);
}
