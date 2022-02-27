static NEIGHBORS_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len() as i32;

    (0..height)
        .map(|i| {
            let width = minefield[i as usize].len() as i32;

            (0..width)
                .map(|j| {
                    if minefield[i as usize].as_bytes()[j as usize] == b'*' {
                        '*'
                    } else {
                        let neighbors_mine_count = NEIGHBORS_OFFSETS
                            .iter()
                            .map(|&(offset_i, offset_j)| (i + offset_i, j + offset_j))
                            .filter(|&(i, j)| (j >= 0 && j < width) && (i >= 0 && i < height))
                            .filter(|&(i, j)| minefield[i as usize].as_bytes()[j as usize] == b'*')
                            .count();

                        match neighbors_mine_count {
                            0 => ' ',
                            n => (n as u8 + '0' as u8) as char,
                        }
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
