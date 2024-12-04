use std::io;
use std::sync::Arc;

fn look(puzzle: &Arc<[Arc<[char]>]>) -> usize{
    let mut counter = 0;
    let (r,c) = (puzzle.len(), puzzle[0].len());
    for i in 0..r {
        for j in 0..c {
            let mut attempts = Vec::<[char;4]>::with_capacity(4);
            //nord
            if i.checked_sub(3).is_some() {
                attempts.push([puzzle[i][j],puzzle[i-1][j],puzzle[i-2][j],puzzle[i-3][j]]);
                // North - West
                if j + 3 < c {
                    attempts.push([puzzle[i][j],puzzle[i-1][j+1],puzzle[i-2][j+2],puzzle[i-3][j+3]]);
                }
                // North - East
                if j.checked_sub(3).is_some() {
                    attempts.push([puzzle[i][j],puzzle[i-1][j-1],puzzle[i-2][j-2],puzzle[i-3][j-3]]);
                }
            }
            //South
            if i + 3 < r  {
                attempts.push([puzzle[i][j],puzzle[i+1][j],puzzle[i+2][j],puzzle[i+3][j]]);
                // South - West
                if j + 3 < c {
                    attempts.push([puzzle[i][j],puzzle[i+1][j+1],puzzle[i+2][j+2],puzzle[i+3][j+3]]);
                }
                // South - East
                if j.checked_sub(3).is_some() {
                    attempts.push([puzzle[i][j],puzzle[i+1][j-1],puzzle[i+2][j-2],puzzle[i+3][j-3]]);
                }
            }
            // West
            if j + 3 < c {
                attempts.push([puzzle[i][j],puzzle[i][j+1],puzzle[i][j+2],puzzle[i][j+3]]);
            }
            // East
            if j.checked_sub(3).is_some() {
                attempts.push([puzzle[i][j],puzzle[i][j-1],puzzle[i][j-2],puzzle[i][j-3]]);
            }
            counter += attempts.iter().filter(|a| {
                match a { 
                    ['X','M','A','S'] => true,
                     _=> false
                }
            }).count();
        }
    }
    return counter;
}
fn main() {
	let puzzle = io::stdin().lines().map(|line| line.unwrap().chars().collect::<Arc<[char]>>()).collect::<Arc<_>>();
    println!("Result: {:?}",look(&puzzle));
} 
