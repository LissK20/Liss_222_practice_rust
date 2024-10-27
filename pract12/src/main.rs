use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32>
{
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize)
{
    let mut min_sum = i32::MAX;
    let mut indices = (0, 0);

    for i in 0..data.len() - 1
    {
        let sum = data[i] + data[i + 1];
        if sum < min_sum
        {
            min_sum = sum;
            indices = (i, i + 1);
        }
    }

    (min_sum, indices.0, indices.1)
}

fn print_vector(data: &[i32])
{
    println!("indexes: {}", (0..data.len()).map(|i| format!("{}.", i)).collect::<Vec<_>>().join("  "));
    println!("data:    {:?}", data);
}

fn main()
{
    for _ in 0..4
    {
        let random_vector = gen_random_vector(20);
        print_vector(&random_vector);

        let (min_sum, idx1, idx2) = min_adjacent_sum(&random_vector);
        println!("indexes: {}  {}", idx1, idx2);
        println!("min adjacent sum={}+{}={} at indexes:{},{}", random_vector[idx1], random_vector[idx2], min_sum, idx1, idx2);
        println!();
    }
}
