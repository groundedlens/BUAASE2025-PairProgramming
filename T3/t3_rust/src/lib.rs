use std::collections::{BinaryHeap, HashSet, HashMap, VecDeque};
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    x: i32,
    y: i32,
    cost: i32,
    heuristic: i32,
    parent_dir: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[wasm_bindgen]
pub fn greedy_snake_step(
    board_size: i32,
    snake: &[i32],
    snake_num: i32,
    other_snakes: &[i32],
    food_num: i32,
    foods: &[i32],
    round: i32,
) -> i32 {
    if snake[0] == -1 {
        return 0;
    }

    // 解析自身蛇信息
    let snake_head = (snake[0], snake[1]);
    let snake_second = if snake.len() >= 4 { (snake[2], snake[3]) } else { (-1, -1) };

    // 修正障碍物预测逻辑
    let obstacles = get_obstacles(other_snakes, board_size);

    // 生成食物列表
    let foods: Vec<(i32, i32)> = foods.chunks(2).map(|c| (c[0], c[1])).collect();

    // 路径规划策略
    if let Some(dir) = advanced_pathfinding(
        snake_head,
        snake_second,
        &foods,
        &obstacles,
        board_size
    ) {
        return dir;
    }

    // 防御性移动策略
    defensive_move(snake_head, snake_second, &obstacles, board_size)
}

// 修正障碍物生成函数
fn get_obstacles(other_snakes: &[i32], board_size: i32) -> HashSet<(i32, i32)> {
    let mut obstacles = HashSet::new();
    for snake_coords in other_snakes.chunks(8) {
        if snake_coords[0] == -1 {
            continue;
        }
        // 添加有效的前三节身体
        for chunk in snake_coords.chunks(2).take(3) {
            if is_valid(chunk[0], chunk[1], board_size) {
                obstacles.insert((chunk[0], chunk[1]));
            }
        }
        // 预测有效移动位置
        let head = (snake_coords[0], snake_coords[1]);
        for (dx, dy) in [(0,1), (-1,0), (0,-1), (1,0)] {
            let nx = head.0 + dx;
            let ny = head.1 + dy;
            if is_valid(nx, ny, board_size) {
                obstacles.insert((nx, ny));
            }
        }
    }
    obstacles
}

// 增强路径规划算法
fn advanced_pathfinding(
    start: (i32, i32),
    snake_second: (i32, i32),
    foods: &[(i32, i32)],
    obstacles: &HashSet<(i32, i32)>,
    board_size: i32,
) -> Option<i32> {
    let mut open = BinaryHeap::new();
    let mut visited = HashMap::new();
    
    // 选择最近且可达的食物
    let target = foods.iter()
        .filter(|&&food| has_path(start, food, obstacles, board_size))
        .min_by_key(|&&food| manhattan(start.0, start.1, food.0, food.1))
        .unwrap_or(&foods[0]);

    open.push(Node {
        x: start.0,
        y: start.1,
        cost: 0,
        heuristic: enhanced_heuristic(start.0, start.1, target.0, target.1, obstacles),
        parent_dir: -1,
    });

    while let Some(current) = open.pop() {
        if (current.x, current.y) == *target {
            return Some(current.parent_dir);
        }

        for (dir_idx, &(dx, dy)) in [(0,1), (-1,0), (0,-1), (1,0)].iter().enumerate() {
            let (nx, ny) = (current.x + dx, current.y + dy);
            
            if is_valid_move((nx, ny), snake_second, obstacles, board_size) 
                && has_escape_routes(nx, ny, obstacles, board_size) 
            {
                let new_cost = current.cost + 1;
                let heuristic = enhanced_heuristic(nx, ny, target.0, target.1, obstacles);
                let parent_dir = if current.parent_dir == -1 { dir_idx as i32 } else { current.parent_dir };
                
                if !visited.contains_key(&(nx, ny)) || visited[&(nx, ny)] > new_cost {
                    visited.insert((nx, ny), new_cost);
                    open.push(Node { x: nx, y: ny, cost: new_cost, heuristic, parent_dir });
                }
            }
        }
    }
    None
}

// 增强防御性移动策略
fn defensive_move(
    head: (i32, i32),
    snake_second: (i32, i32),
    obstacles: &HashSet<(i32, i32)>,
    board_size: i32,
) -> i32 {
    let mut best_dir = 0;
    let mut max_space = -1;
    let mut has_valid = false;

    for (dir_idx, &(dx, dy)) in [(0,1), (-1,0), (0,-1), (1,0)].iter().enumerate() {
        let (nx, ny) = (head.0 + dx, head.1 + dy);
        if is_valid_move((nx, ny), snake_second, obstacles, board_size) {
            has_valid = true;
            let space = flood_fill_space(nx, ny, obstacles, board_size);
            if space > max_space {
                max_space = space;
                best_dir = dir_idx as i32;
            }
        }
    }

    // 无有效方向时选择第一个合法移动
    if !has_valid {
        for (dir_idx, &(dx, dy)) in [(0,1), (-1,0), (0,-1), (1,0)].iter().enumerate() {
            let (nx, ny) = (head.0 + dx, head.1 + dy);
            if is_valid(nx, ny, board_size) { // 至少保证不撞墙
                return dir_idx as i32;
            }
        }
    }
    best_dir
}

// 碰撞检测（确保边界检查优先）
fn is_valid_move(
    new_head: (i32, i32),
    snake_second: (i32, i32),
    obstacles: &HashSet<(i32, i32)>,
    board_size: i32,
) -> bool {
    is_valid(new_head.0, new_head.1, board_size) && // 边界检查
    !obstacles.contains(&new_head) &&
    new_head != snake_second
}

// 逃生路线检测
fn has_escape_routes(
    x: i32,
    y: i32,
    obstacles: &HashSet<(i32, i32)>,
    board_size: i32,
) -> bool {
    let mut escape_count = 0;
    for (dx, dy) in [(0,1), (-1,0), (0,-1), (1,0)] {
        let (nx, ny) = (x + dx, y + dy);
        if is_valid(nx, ny, board_size) && !obstacles.contains(&(nx, ny)) {
            escape_count += 1;
        }
    }
    escape_count >= 2
}

// 洪水填充计算可用空间
fn flood_fill_space(x: i32, y: i32, obstacles: &HashSet<(i32, i32)>, board_size: i32) -> i32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    visited.insert((x, y));

    while let Some((cx, cy)) = queue.pop_front() {
        for (dx, dy) in [(0,1), (-1,0), (0,-1), (1,0)] {
            let (nx, ny) = (cx + dx, cy + dy);
            if is_valid(nx, ny, board_size)
                && !obstacles.contains(&(nx, ny))
                && !visited.contains(&(nx, ny))
            {
                visited.insert((nx, ny));
                queue.push_back((nx, ny));
            }
        }
    }
    visited.len() as i32
}

// 增强启发函数
fn enhanced_heuristic(x: i32, y: i32, tx: i32, ty: i32, dangers: &HashSet<(i32, i32)>) -> i32 {
    let base = manhattan(x, y, tx, ty);
    let danger_penalty = if dangers.contains(&(x, y)) { 50 } else { 0 };
    base + danger_penalty
}

// 路径可达性检查
fn has_path(start: (i32, i32), end: (i32, i32), obstacles: &HashSet<(i32, i32)>, board_size: i32) -> bool {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == end {
            return true;
        }
        for (dx, dy) in [(0,1), (-1,0), (0,-1), (1,0)] {
            let (nx, ny) = (x + dx, y + dy);
            if is_valid(nx, ny, board_size)
                && !obstacles.contains(&(nx, ny))
                && !visited.contains(&(nx, ny))
            {
                visited.insert((nx, ny));
                queue.push_back((nx, ny));
            }
        }
    }
    false
}

// 基础工具函数
fn manhattan(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn is_valid(x: i32, y: i32, board_size: i32) -> bool {
    x >= 1 && x <= board_size && y >= 1 && y <= board_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
