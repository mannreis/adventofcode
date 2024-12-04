use std::io;
use std::sync::Arc;

fn look(puzzle: &Arc<[Arc<[char]>]>) -> usize{
    let (r,c) = (puzzle.len(), puzzle[0].len());
    let mut valid = Vec::<(usize,usize)>::with_capacity(10);
    for i in 0..r {
        for j in 0..c{
            if puzzle[i][j] != 'A'{
                continue;
            }
            if i.checked_sub(1).is_some() && i + 1 < c 
                && j.checked_sub(1).is_some() && j + 1 < r{
                if ([ puzzle[i-1][j-1],puzzle[i][j],puzzle[i+1][j+1] ] == ['M','A','S'] ||
                    [ puzzle[i-1][j-1],puzzle[i][j],puzzle[i+1][j+1] ] == ['S','A','M']) &&
                   ([ puzzle[i+1][j-1],puzzle[i][j],puzzle[i-1][j+1] ] == ['M','A','S'] ||
                    [ puzzle[i+1][j-1],puzzle[i][j],puzzle[i-1][j+1] ] == ['S','A','M']) {
                        valid.push((i,j));
                }
            }
        }
    }
    return valid.len();
}

fn main() {
	let puzzle = io::stdin().lines().map(|line| line.unwrap().chars().collect::<Arc<[char]>>()).collect::<Arc<_>>();
    println!("Result: {:?}",look(&puzzle));
} 
