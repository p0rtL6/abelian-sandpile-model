use std::{fs::File, io::Write};

fn main() {
    let starting_value: usize = 10000000;

    let grid_size: usize = (starting_value as f64).sqrt() as usize + 1;

    let mut grid = vec![vec![0usize; grid_size]; grid_size];
    let center = grid_size / 2;

    let mut range = 0;

    grid[center][center] = starting_value;

    // let mut frame_counter = 0;

    // for frame in 0..starting_value {
        // grid[center][center] += 1;
        loop {
            let mut changed = false;
            for row in center - range..=center + range {
                for col in center - range..=center + range {
                    if grid[row][col] > 3 {
                        changed = true;
                        let overflow = grid[row][col] / 4;
                        grid[row][col] %= 4;

                        grid[row][col - 1] += overflow;
                        grid[row][col + 1] += overflow;
                        grid[row - 1][col] += overflow;
                        grid[row + 1][col] += overflow;

                        if row == center - range || row == center + range {
                            range += 1;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
            // frame_counter += 1;
        }
        create_ppm(format!("./output/piles-{}.ppm", starting_value), &grid);
    // }
}

fn create_ppm(filename: String, grid: &Vec<Vec<usize>>) {
    let grid_size = grid.len();
    let header = format!("P6 {} {} 255\n", grid_size, grid_size);
    let mut buffer = vec![0u8; grid_size * grid_size * 3];

    let mut offset = 0;

    for row in grid {
        for item in row {
            // let mut rgb_shade = 0;
            match item {
                0 => {
                    buffer[offset] = 245;
                    offset += 1;
                    buffer[offset] = 227;
                    offset += 1;
                    buffer[offset] = 224;
                    offset += 1;
                }
                1 => {
                    buffer[offset] = 232;
                    offset += 1;
                    buffer[offset] = 180;
                    offset += 1;
                    buffer[offset] = 188;
                    offset += 1;
                }
                2 => {
                    buffer[offset] = 210;
                    offset += 1;
                    buffer[offset] = 130;
                    offset += 1;
                    buffer[offset] = 166;
                    offset += 1;
                }
                3 => {
                    buffer[offset] = 110;
                    offset += 1;
                    buffer[offset] = 69;
                    offset += 1;
                    buffer[offset] = 85;
                    offset += 1;
                }
                _ => {}
            }

            // for _ in 0..3 {
            //     buffer[offset] = rgb_shade;
            //     offset += 1;
            // }
        }
    }

    let mut file = File::create(filename).unwrap();

    file.write(header.as_bytes()).unwrap();
    file.write(&buffer).unwrap();
}
