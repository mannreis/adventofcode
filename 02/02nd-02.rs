use std::io;

fn m_reduce<T:Clone + std::ops::Mul<Output = T> + Copy>(v: Vec<T>) -> T {
    let mut r :T = v[0].clone();
    v[1..].iter().for_each(|&x| r = r * x);
    r
}
fn main() {
    let lines = std::io::stdin().lines();
    let mut sum = 0;
    for line in lines {
        // parse line
        let line = line.unwrap();
        let (game, sets) = line.split_once(':').unwrap();

        // get game number
        let mut _game_it = game.split_whitespace();
        _game_it.next();
        let mut game: u32 = _game_it.next().unwrap().parse().unwrap();

        print!("{} > ", game);

        let mut game_power = [-1, -1, -1];
        // parse game sets
        for set in sets.split(";") {
            let invalid_set = set.split(',').for_each(|x| {
                let (a, c) = x[1..].split_once(' ').unwrap();
                let a: i32 = a.parse().unwrap();
                let color_index = match c {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => 3,
                };
                assert!(color_index < 3);
                game_power[color_index] =
                    if game_power[color_index] == -1 || game_power[color_index] < a {
                        a
                    } else {
                        game_power[color_index]
                    };
            });
            print!("{} ", set);
            print!("[{:?}]", game_power);
        }
        let power = m_reduce(game_power.to_vec());
        sum += power;
        println!("{:?} = {}", game_power,power);
    }
    println!("{}", sum);
}
