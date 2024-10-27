fn count_permutation(shipments: &Vec<u32>) -> usize
{
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0
    {
        return usize::MAX;
    }

    let average = total / n;
    let mut moves = 0;
    let mut current_diff = 0;

    for &shipment in shipments
    {
        current_diff += shipment as i32 - average as i32;
        moves += current_diff.abs();
    }

    moves as usize
}

fn gen_shipments(n: usize) -> Vec<u32>
{
    let average = 4;
    let mut shipments = vec![average; n];
    let total: u32 = average * n as u32;
    let mut extra = total % 2;

    for i in 0..n
    {
        if extra > 0
        {
            shipments[i] += 1;
            extra -= 1;
        }
    }

    shipments
}

fn main()
{
    let shipments1 = vec![8, 2, 2, 4, 4];
    let answer1 = count_permutation(&shipments1);
    if answer1 != usize::MAX
    {
        println!("Мінімальна кількість переносу для {:?} = {}", shipments1, answer1);
    }
    else
    {
        println!("Неможливо рівномірно розподілити для {:?}", shipments1);
    }

    let shipments2 = vec![9, 3, 7, 2, 9];
    let answer2 = count_permutation(&shipments2);
    if answer2 != usize::MAX
    {
        println!("Мінімальна кількість переносу для {:?} = {}", shipments2, answer2);
    }
    else
    {
        println!("Неможливо рівномірно розподілити для {:?}", shipments2);
    }

    let generated_shipments = gen_shipments(5);
    println!("Згенеровані відправлення: {:?}", generated_shipments);
}
