use std::io;

fn check_possible(color: &str, amount: u32) -> bool {
    match color {
        "red" => amount <= 12,
        "green" => amount <= 13,
        "blue" => amount <= 14,
        _ => false,
    }
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
        
        // parse game sets
        for set in sets.split(";") {
            let invalid_set = set.split(',').any(|x| {
                //print!(">{}<",&x[1..]);
                let (a, c) = &x[1..].split_once(' ').unwrap();
                let a: u32 = a.parse().unwrap();
                !check_possible(c, a)
            });
            print!("{} ", set);
            if invalid_set {
                print!("[NO]");
                // go to next game!
                game = 0;
                break;
            } else {
                print!("[YES]");
            }
        }
        sum += game;
        println!();
    }
    println!("{}",sum);
}
