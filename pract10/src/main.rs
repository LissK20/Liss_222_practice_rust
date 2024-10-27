fn rotate(s: String, n: isize) -> String
{
    let len = s.len() as isize;
    if len == 0
    {
        return s;
    }

    let n = n.rem_euclid(len) as usize;

    let split_index = (len - n as isize) as usize;
    let (first, second) = s.split_at(split_index);

    format!("{}{}", second, first)
}

fn main()
{
    let s = "abcdefgh".to_string();
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    for (n, exp) in shifts.iter()
    {
        let result = rotate(s.clone(), *n);
        println!("rotate({}, {}) = {}", s, n, result);
        assert_eq!(result, exp.to_string());
    }
}
