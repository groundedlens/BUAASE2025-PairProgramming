use std::collections::{HashSet, VecDeque};
use wasm_bindgen::prelude::*;


const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(snake: &[i32], fruit: &[i32], barriers: &[i32]) -> i32 {
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

    let candidates = candidates_generation(head, body, &barrier_set);

    if candidates.is_empty() {
        return -1;
    }

    let mut reachable = Vec::new();  // (方向码, bfs步数)
    let mut unreachable = Vec::new(); // (方向码, 自由度)
    
    for (code, new_head, new_body) in &candidates {
        let all_blocks = merge_blocks(&barrier_set, new_body);
        match bfs(*new_head, target, &all_blocks) {
            Some(steps) => reachable.push((*code, steps)),
            None => {
                let freedom = calculate_freedom(*new_head, &all_blocks);
                unreachable.push((*code, freedom));
            }
        }
    }

    if !reachable.is_empty() {
        return select_best_reachable(reachable);
    }

    // 没有可达路径，进行模拟
    let best_code = select_best_unreachable(unreachable);
    if best_code == -1 {
        return -1;
    }

    // 模拟移动，最多50步
    let mut simulated_snake = snake.to_vec();
    // 找到对应的候选方向以初始化模拟蛇的状态
    let (new_head, new_body) = candidates.iter()
        .find(|&&(code, _, _)| code == best_code)
        .map(|&(_, nh, nb)| (nh, nb))
        .unwrap();

    // 更新模拟的蛇数组
    simulated_snake = vec![
        new_head.0, new_head.1,
        new_body[0].0, new_body[0].1,
        new_body[1].0, new_body[1].1,
        new_body[2].0, new_body[2].1,
    ];

    let mut steps_remaining = 50;
    let mut current_barrier_set = barrier_set.clone();

    loop {
        let current_head = (simulated_snake[0], simulated_snake[1]);
        let current_body = [
            (simulated_snake[2], simulated_snake[3]),
            (simulated_snake[4], simulated_snake[5]),
            (simulated_snake[6], simulated_snake[7]),
        ];

        let candidates = candidates_generation(current_head, current_body, &current_barrier_set);
        if candidates.is_empty() {
            break;
        }

        let mut sim_reachable = Vec::new();
        let mut sim_unreachable = Vec::new();

        for (code, nh, nb) in &candidates {
            let all_blocks = merge_blocks(&current_barrier_set, nb);
            match bfs(*nh, target, &all_blocks) {
                Some(steps) => sim_reachable.push((*code, steps)),
                None => {
                    let freedom = calculate_freedom(*nh, &all_blocks);
                    sim_unreachable.push((*code, freedom));
                }
            }
        }

        if !sim_reachable.is_empty() {
            return best_code;
        }

        if sim_unreachable.is_empty() {
            break;
        }

        // 选择最优的不可达方向
        sim_unreachable.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let next_code = sim_unreachable[0].0;

        // 更新模拟的蛇数组
        let (updated_head, updated_body) = candidates.iter()
            .find(|&&(code, _, _)| code == next_code)
            .map(|&(_, nh, nb)| (nh, nb))
            .unwrap();

        simulated_snake = vec![
            updated_head.0, updated_head.1,
            updated_body[0].0, updated_body[0].1,
            updated_body[1].0, updated_body[1].1,
            updated_body[2].0, updated_body[2].1,
        ];

        // 检查移动后是否可达
        let all_blocks = merge_blocks(&current_barrier_set, &updated_body);
        if bfs(updated_head, target, &all_blocks).is_some() {
            return best_code;
        }

        steps_remaining -= 1;
        if steps_remaining == 0 {
            break;
        }
    }

    -1
}


fn candidates_generation(
    head: (i32, i32),
    body: [(i32, i32); 3],
    barriers: &HashSet<(i32, i32)>
) -> Vec<(i32, (i32, i32), [(i32, i32); 4])> {
    let mut candidates = Vec::new();
    for (dir_code, &(dx, dy)) in DIRECTIONS.iter().enumerate() {
        let new_head = (head.0 + dx, head.1 + dy);
        
        if new_head.0 < 1 || new_head.0 > 8 || new_head.1 < 1 || new_head.1 > 8 {
            continue;
        }
        
        if barriers.contains(&new_head) {
            continue;
        }
        
        let new_body = [new_head, head, body[0], body[1]];
        
        if new_body[1..].contains(&new_head) {
            continue;
        }
        
        candidates.push((dir_code as i32, new_head, new_body));
    }
    candidates
}

fn merge_blocks(
    barriers: &HashSet<(i32, i32)>,
    body: &[(i32, i32); 4]
) -> HashSet<(i32, i32)> {
    barriers.iter().copied().chain(body.iter().copied()).collect()
}

fn select_best_reachable(mut reachable: Vec<(i32, i32)>) -> i32 {
    reachable.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    reachable[0].0
}

fn select_best_unreachable(mut unreachable: Vec<(i32, i32)>) -> i32 {
    if unreachable.is_empty() {
        return -1;
    }
    unreachable.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    unreachable[0].0
}

fn calculate_freedom(pos: (i32, i32), blocks: &HashSet<(i32, i32)>) -> i32 {
    DIRECTIONS.iter().filter(|&&(dx, dy)| {
        let new_pos = (pos.0 + dx, pos.1 + dy);
        new_pos.0 >= 1 && new_pos.0 <= 8 &&
        new_pos.1 >= 1 && new_pos.1 <= 8 &&
        !blocks.contains(&new_pos)
    }).count() as i32
}

fn bfs(
    start: (i32, i32),
    end: (i32, i32),
    blocks: &HashSet<(i32, i32)>
) -> Option<i32> {
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
    fn test_simple_path() { // 简单路径
        let snake = [7, 5, 8, 5, 8, 6, 8, 7];
        let fruit = [1,1];
        let barriers = [6, 1, 6, 2, 6, 3, 6, 4, 7, 4, 4, 5, 5, 5, 5, 6, 3, 6, 3, 7, 2, 7, 5, 8];
        assert_ne!(greedy_snake_move_barriers(&snake, &fruit, &barriers), 2);
        assert_ne!(greedy_snake_move_barriers(&snake, &fruit, &barriers), 3);
    }

    #[test]
    fn test_hard_path() { // 需要掉头的情况
        let snake_body = [8, 4, 8, 5, 8, 6, 8, 7];
        let food = [1, 1];
        let barriers = [6, 1, 6, 2, 6, 3, 6, 4, 7, 4, 4, 5, 5, 5, 5, 6, 3, 6, 3, 7, 2, 7, 5, 8];
        assert_eq!(greedy_snake_move_barriers(&snake_body, &food, &barriers), 2);
    }

    #[test]
    fn test2() { // 需要掉头的情况
        let snake_body = [8, 3, 8, 4, 8, 5, 8, 6];
        let food = [1, 1];
        let barriers = [6, 1, 6, 2, 6, 3, 6, 4, 7, 3, 4, 5, 5, 5, 5, 6, 3, 6, 3, 7, 2, 7, 5, 8];
        assert_eq!(greedy_snake_move_barriers(&snake_body, &food, &barriers), 2);
    }

    #[test]
    fn test3() {
        let snake_body = [8, 2,  8, 3,  8, 4,  8, 5];
        let food = [1, 1];
        let barriers = [6, 1,  6, 2,  6, 3,  6, 4,  7, 2,  4, 5, 5, 5, 5, 6, 3, 6, 3, 7, 2, 7, 5, 8];
        assert_eq!(greedy_snake_move_barriers(&snake_body, &food, &barriers), -1);
    }

    #[test]
    fn test_unreachable_path() { //应当返回-1（压根走不出去）的情况
        let snake = [8, 4, 8, 5, 8, 6, 8, 7];
        let fruit = [1, 1];
        let barriers = [6, 1, 6, 2, 6, 3, 6, 4, 7, 4, 4, 5, 5, 5, 5, 6, 5, 7, 3, 6, 3, 7, 5, 8];
        assert_eq!(greedy_snake_move_barriers(&snake, &fruit, &barriers), -1);
    }
}
