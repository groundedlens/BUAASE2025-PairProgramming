use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greedy_snake_move(snake: Vec<i32>, food: Vec<i32>) -> i32 {
    let head = [snake[0], snake[1]];
    let body1 = [snake[2], snake[3]];
    let body2 = [snake[4], snake[5]];
    let body3 = [snake[6], snake[7]];
    
    let up = [head[0], head[1] + 1];
    let down = [head[0], head[1] - 1];
    let left = [head[0] - 1, head[1]];
    let right = [head[0] + 1, head[1]];
    
    let choices = [up, left, down, right];
    let mut ans = -1;
    let mut min_distance = i32::MAX;
    
    for (i, &[x, y]) in choices.iter().enumerate() {
        // 检查边界
        if x < 1 || x > 8 || y < 1 || y > 8 {
            continue;
        }
        
        // 检查是否碰到身体
        if [x, y] == body1 || [x, y] == body2 || [x, y] == body3 {
            continue;
        }
        
        // 计算距离
        let distance = (x - food[0]).pow(2) + (y - food[1]).pow(2);
        if distance < min_distance {
            min_distance = distance;
            ans = i as i32;
        }
    }
    
    ans
}



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
