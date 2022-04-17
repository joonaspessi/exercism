pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let parsed: Vec<&[u8]> = minefield.iter().map(|line| line.as_bytes()).collect();
    let mut result: Vec<String> = vec![];

    for y in 0..parsed.len() {
        let mut y_result = String::new();
        for x in 0..parsed[0].len() {
            if parsed[y][x] == b' ' {
                let count = count_neighbors(x as i32, y as i32, &parsed);
                if count > 0 {
                    y_result.push(char::from_digit(count, 10).unwrap());
                } else {
                    y_result.push(' ')
                }
            } else if parsed[y][x] == b'*' {
                y_result.push('*');
            }
        }
        result.push(y_result);
    }
    result
}

fn count_neighbors(x: i32, y: i32, map: &[&[u8]]) -> u32 {
    let mut count = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            let yy: i32 = dy + y;
            let xx: i32 = dx + x;

            if yy >= 0
                && xx >= 0
                && yy < map.len() as i32
                && xx < map[0].len() as i32
                && !(dy == 0 && dx == 0)
                && map[yy as usize][xx as usize] == b'*'
            {
                count += 1;
            }
        }
    }
    count
}
