use std::collections::{HashSet, VecDeque};
use wasm_bindgen::prelude::*;


const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(snake: &[i32], fruit: &[i32], barriers: &[i32],) -> i32 {
    let head = (snake[0], snake[1]);
    let body = [
        (snake[2], snake[3]),
        (snake[4], snake[5]),
        (snake[6], snake[7]),
    ];
    let target = (fruit[0], fruit[1]);
    
    let mut barrier_set = HashSet::new();
    for i in (0..barriers.len()).step_by(2) {
        barrier_set.insert((barriers[i], barriers[i+1]));
    }

    let mut candidates = Vec::new();
    for dir_code in 0..4 {
        let (dx, dy) = DIRECTIONS[dir_code];
        let new_head = (head.0 + dx, head.1 + dy);
        
        if new_head.0 < 1 || new_head.0 > 8 || new_head.1 < 1 || new_head.1 > 8 {
            continue;
        }
        if barrier_set.contains(&new_head) {
            continue;
        }
        let new_body = [new_head, head, body[0], body[1]];

        let mut collision = false;
        for i in 1..4 {
            if new_body[i] == new_head {
                collision = true;
                break;
            }
        }
        if collision {
            continue;
        }
        candidates.push((dir_code as i32, new_head, new_body));
    }

    if candidates.is_empty() {
        return -1;
    }

    let mut best_dir = -1;
    let mut min_steps = i32::MAX;
    
    for (code, new_head, new_body) in candidates {
        let mut all_blocks = barrier_set.clone();
        for pos in &new_body {
            all_blocks.insert(*pos);
        }
        
        match bfs(new_head, target, &all_blocks) {
            Some(steps) => {
                if steps < min_steps || (steps == min_steps && code < best_dir) {
                    min_steps = steps;
                    best_dir = code;
                }
            }
            None => continue
        }
    }
    
    best_dir
}


fn bfs(start: (i32, i32), end: (i32, i32), blocks: &HashSet<(i32, i32)>) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));
    visited.insert(start);
    
    while let Some((x, y, steps)) = queue.pop_front() {
        if (x, y) == end {
            return Some(steps);
        }
        
        for (dx, dy) in &DIRECTIONS {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 1 || nx > 8 || ny < 1 || ny > 8 {
                continue;
            }
            if blocks.contains(&(nx, ny)) {
                continue;
            }
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }
    
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let snake_body = [8, 4, 8, 5, 8, 6, 8, 7];
        let food = [1, 1];
        let barriers = [6, 1, 6, 2, 6, 3, 6, 4, 7, 4, 4, 5, 5, 5, 5, 6, 3, 6, 3, 7, 2, 7, 5, 8];
        assert_eq!(greedy_snake_move_barriers(&snake_body, &food, &barriers), 2);
    }

    #[test]
    fn test2() {
        let snake_body = [8, 3, 8, 4, 8, 5, 8, 6];
        let food = [1, 1];
        let barriers = [6, 1, 6, 2, 6, 3, 6, 4, 7, 3, 4, 5, 5, 5, 5, 6, 3, 6, 3, 7, 2, 7, 5, 8];
        assert_eq!(greedy_snake_move_barriers(&snake_body, &food, &barriers), -1);
    }

    #[test]
    fn test3() {
        let snake_body = [8, 7, 8, 6, 8, 5, 8, 4];
        let food = [1, 1];
        let barriers = [6, 1, 6, 2, 6, 3, 6, 4, 7, 4, 4, 5, 5, 5, 5, 6, 3, 6, 3, 7, 2, 7, 5, 8];
        assert_eq!(greedy_snake_move_barriers(&snake_body, &food, &barriers), 1);
    }
}
