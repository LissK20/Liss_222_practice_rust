use std::collections::HashSet;

fn find_variables() -> Vec<(i32, i32, i32, i32, i32, i32, i32, i32)>
{
    let mut solutions = Vec::new();
    for m in 1..=8
    {
        for u in 1..=8
        {
            for x in 1..=8
            {
                for a in 1..=8
                {
                    for s in 1..=8
                    {
                        for l in 1..=8
                        {
                            for o in 1..=8
                            {
                                for n in 1..=8
                                {
                                    let unique_numbers: HashSet<_> =
                                        vec![m, u, x, a, s, l, o, n].into_iter().collect();
                                    if unique_numbers.len() == 8
                                    {
                                        let left = m * 1000 + u * 100 + x * 10 + a;
                                        let right = s * 1000 + l * 100 + o * 10 + n;
                                        if left == 10 * right
                                        {
                                            solutions.push((m, u, x, a, s, l, o, n));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    solutions
}

fn print_solution(m: i32, u: i32, x: i32, a: i32, s: i32, l: i32, o: i32, n: i32)
{
    println!("{:>4}", m);
    println!("{:>4}", u);
    println!("{:>4}", x);
    println!("{:>4}", a);
    println!("------");
    println!("{:>4}", s);
    println!("{:>4}", l);
    println!("{:>4}", o);
    println!("{:>4}", n);
}

fn main()
{
    let solutions = find_variables();
    for (m, u, x, a, s, l, o, n) in &solutions
    {
        print_solution(*m, *u, *x, *a, *s, *l, *o, *n);
        println!();
    }
    println!("Total solutions: {}", solutions.len());
}
