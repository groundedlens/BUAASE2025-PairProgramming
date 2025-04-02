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
        
        // 检查是否碰到身体(实际上可以往body3走)
        if [x, y] == body1 {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_snack_move1() {
        let snake = vec![1, 1, 2, 1, 3, 1, 4, 1];
        let food: Vec<i32> = vec![1, 2];
        let result: i32 = greedy_snake_move(snake, food);
        assert_ne!(result, 1);
        assert_ne!(result, 2);
        assert_ne!(result, 3);
    }

    #[test]
    fn test_greedy_snack_move2() {
        let snake: Vec<i32> = vec![3, 4, 3, 3, 3, 2, 2, 2];
        let food: Vec<i32> = vec![5, 6];
        let result: i32 = greedy_snake_move(snake, food);
        assert_ne!(result, 3);
    }

    #[test]
    fn test_greedy_snack_move3() {
        let snake: Vec<i32> = vec![5, 6, 5, 5, 5, 4, 5, 3];
        let food: Vec<i32> = vec![5, 2];
        let result: i32 = greedy_snake_move(snake, food);
        assert_ne!(result, 3);
    }
}
