struct Point
{
    x: i32,
    y: i32,
}

struct Rectangle
{
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32
{
    let mut occupied = vec![vec![false; 100]; 100];
    let mut total_area = 0;

    for rect in xs
    {
        let (x1, x2) = (rect.a.x.min(rect.b.x), rect.a.x.max(rect.b.x));
        let (y1, y2) = (rect.a.y.min(rect.b.y), rect.a.y.max(rect.b.y));

        for x in x1..x2
        {
            for y in y1..y2
            {
                if !occupied[y as usize][x as usize]
                {
                    occupied[y as usize][x as usize] = true;
                    total_area += 1;
                }
            }
        }
    }

    total_area
}

fn test_data() -> Vec<Rectangle>
{
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test()
{
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main()
{
    area_occupied_test();
}
