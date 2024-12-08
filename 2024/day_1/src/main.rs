fn main() {
    // read input file into 2 arrays
    let input = std::fs::read_to_string("src/input_big.txt").unwrap();
    let (mut numbers1, mut numbers2): (Vec<_>, Vec<_>) = input
    .lines()
    .map(|x| {
        let mut numbersplit = x.split_whitespace();
        (
            numbersplit.next().unwrap().parse::<i32>().unwrap(),
            numbersplit.next().unwrap().parse::<i32>().unwrap(),
        )
    })
    .unzip();

    numbers1.sort();
    numbers2.sort();
    let mut total_distance: i32 = 0;
    numbers1.iter().zip(numbers2.iter()).for_each(|(x, y)| {
        total_distance += (y - x).abs();
    }
    );
    print!("Total distance: {}\n", total_distance);

    let mut total_similarity: i32 = 0;
    numbers1.iter().for_each(|x| {
        total_similarity += x * numbers2.iter().filter(|y| x == *y).count() as i32;
    });
    print!("Total similarity: {}\n", total_similarity);
}
