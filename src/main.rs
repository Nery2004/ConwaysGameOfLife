use raylib::prelude::*;

const WIDTH: i32 = 840;
const HEIGHT: i32 = 880;
const CELL: i32 = 20;
const COLS: usize = (WIDTH / CELL) as usize;
const ROWS: usize = (HEIGHT / CELL) as usize;
const WHITE: Color = Color::WHITE;
const BLACK: Color = Color::BLACK;

fn init_grid() -> Vec<Vec<bool>> {
    let mut g = vec![vec![false; COLS]; ROWS];
    let p = vec![
        (10, 10), (10, 11), (11, 10), (11, 11),
        (30, 12), (31, 13), (31, 14), (30, 14), (29, 14),
        (20, 20), (21, 21), (22, 21), (22, 20), (22, 19),
        (15, 25), (15, 26), (15, 27),
        (25, 5), (25, 6), (25, 7), (24, 7), (23, 6),
        (35, 30), (36, 30), (37, 30), (38, 30), (39, 30),
        (5, 35), (6, 35), (6, 36), (7, 36), (7, 37),
        (15, 15), (15, 16), (16, 15), (16, 16),
        (25, 25), (26, 26), (27, 26), (27, 25), (27, 24),
        (5, 12), (6, 13), (6, 14), (5, 14), (4, 14),
        (10, 30), (11, 30), (11, 31), (12, 31), (12, 32),
        (30, 30), (30, 31), (31, 30), (31, 31),
        (35, 10), (35, 11), (35, 12),
        (5, 12), (6, 13), (6, 14), (5, 14), (4, 14),
        (35, 30), (36, 30), (37, 30), (38, 30), (39, 30),
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (0, 3), (0, 4), (0, 8), (0, 9), (0, 10),
        (5, 2), (5, 3), (5, 4), (5, 8), (5, 9), (5, 10),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (12, 2), (12, 3), (12, 4), (12, 8), (12, 9), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
        (5, 7), (7, 5), (7, 7), (5, 5),
        (14, 0), (15, 0), (16, 0), (20, 0), (21, 0), (22, 0),
        (0, 14), (0, 15), (0, 16), (0, 20), (0, 21), (0, 22),
        (10,10), (11,10), (11,11),
        (13,13), (13,12), (12,13)
    ];
    for (x, y) in p {
        if x < COLS && y < ROWS {
            g[y][x] = true;
        }
    }
    g
}

fn draw_grid(d: &mut RaylibDrawHandle, grid: &Vec<Vec<bool>>) {
    for y in 0..ROWS {
        for x in 0..COLS {
            if grid[y][x] {
                d.draw_rectangle(x as i32 * CELL, y as i32 * CELL, CELL, CELL, WHITE);
            }
        }
    }
}

fn count_neighbors(g: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut c = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < COLS as i32 && ny < ROWS as i32 {
                if g[ny as usize][nx as usize] { c += 1; }
            }
        }
    }
    c
}

fn next_gen(g: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut n = vec![vec![false; COLS]; ROWS];
    for y in 0..ROWS {
        for x in 0..COLS {
            let c = count_neighbors(g, x, y);
            n[y][x] = match (g[y][x], c) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    n
}

fn main() {
    let (mut rl, th) = raylib::init().size(WIDTH, HEIGHT).title("Conway en Rust").build();
    rl.set_target_fps(10);
    let mut grid = init_grid();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&th);
        d.clear_background(BLACK);
        draw_grid(&mut d, &grid);
        grid = next_gen(&grid);
    }
}
