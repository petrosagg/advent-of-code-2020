use crate::lib::get_input_filter_all;
use itertools::Itertools;

use std::mem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;


#[derive(Debug)]
struct Tile {
    id: u64,
    data: Vec<Vec<bool>>,
    left: Vec<bool>,
    right: Vec<bool>,
}

impl Tile {
    fn new(id: u64, data: Vec<Vec<bool>>) -> Self {
        let size = data[0].len();
        let mut tile = Self {
            id,
            data,
            left: vec![false; size],
            right: vec![false; size],
        };
        tile.update();
        tile
    }

    fn update(&mut self) {
        for (i, row) in self.data.iter().enumerate() {
            self.left[i] = row[0];
            self.right[i] = row[row.len()-1];
        }
    }

    fn flip(&mut self) {
        for row in self.data.iter_mut() {
            row.reverse();
        }
        self.update();
    }

    fn rotate(&mut self) {
        let len = self.data.len();
        let mut new_data = vec![vec![]; len];

        for row in &self.data {
            for (i, x) in row.iter().enumerate() {
                new_data[len-1-i].push(*x)
            }
        }
        self.data = new_data;
        self.update();
    }

    fn top(&self) -> &[bool] {
        &self.data[0]
    }

    fn bottom(&self) -> &[bool] {
        &self.data[self.data.len()-1]
    }

    fn right(&self) -> &[bool] {
        &self.right
    }

    fn left(&self) -> &[bool] {
        &self.left
    }
}

fn expand(size: usize, row: usize, col: usize, board: &mut Vec<Vec<u64>>, bottom_edge: &mut Vec<Vec<bool>>, right_edge: &[bool], used_tiles: &mut HashSet<u64>, tiles: &[(u64, RefCell<Tile>)]) -> Option<()> {
    if row == size && col == 0 {
        // We're done, exit the search
        return None
    }
    // Find the next piece
    for (id, tile) in tiles {
        if used_tiles.contains(&id) {
            continue
        }
        let mut tile = tile.borrow_mut();
        used_tiles.insert(*id);
        for _ in 0..2 {
            tile.flip();
            for _ in 0..4 {
                tile.rotate();
                board[row][col] = tile.id;

                match (row, col) {
                    (0, 0) => {
                        bottom_edge.clear();
                        bottom_edge.push(tile.bottom().to_vec());
                        expand(size, row, col+1, board, bottom_edge, tile.right(), used_tiles, tiles)?;
                        bottom_edge.pop();
                    },
                    (0, _) => {
                        if tile.left() == right_edge {
                            bottom_edge.push(tile.bottom().to_vec());
                            if col < size - 1 {
                                expand(size, row, col + 1, board, bottom_edge, tile.right(), used_tiles, tiles)?;
                            } else {
                                expand(size, row + 1, 0, board, bottom_edge, tile.right(), used_tiles, tiles)?;
                            }
                            bottom_edge.pop();
                        }
                    },
                    (_, 0) => {
                        if tile.top() == &bottom_edge[0] {
                            bottom_edge[col] = tile.bottom().to_vec();
                            expand(size, row, col + 1, board, bottom_edge, tile.right(), used_tiles, tiles)?;
                        }
                    },
                    (_, _) => {
                        if tile.left() == right_edge && tile.top() == &bottom_edge[col] {
                            bottom_edge[col] = tile.bottom().to_vec();
                            if col < size - 1 {
                                expand(size, row, col + 1, board, bottom_edge, tile.right(), used_tiles, tiles)?;
                            } else {
                                expand(size, row + 1, 0, board, bottom_edge, tile.right(), used_tiles, tiles)?;
                            }
                        }
                    }
                }
            }
        }
        used_tiles.remove(&id);
    }
    // We need to backtrack
    Some(())
}

pub fn first() {
    let tiles = get_input_filter_all(20, 1, {
        let mut id = None;
        let mut tile: Vec<Vec<bool>> = vec![];

        move |l| {
            if id == None {
                let (_, id_str) = l.split_at(5);
                id = Some(id_str[..id_str.len()-1].parse::<u64>().unwrap());
            } else if l == "" {
                let tile = mem::replace(&mut tile, vec![]);
                let id = id.take().unwrap();
                return Some((id, RefCell::new(Tile::new(id, tile))))
            } else {
                tile.push(l.chars().map(|c| c == '#').collect_vec());
            }
            None
        }
    });

    let mut used_tiles = HashSet::new();

    let size = (tiles.len() as f64).sqrt() as usize;

    let mut board = vec![vec![0u64; size]; size];
    let mut bottom_edge = vec![];
    if expand(size, 0, 0, &mut board, &mut bottom_edge, &[], &mut used_tiles, &tiles).is_some() {
        panic!();
    }

    let mut acc = 1;
    acc *= board[0][0];
    acc *= board[0][size-1];
    acc *= board[size-1][0];
    acc *= board[size-1][size-1];
    
    println!("part1={}", acc);

    let tiles = tiles.into_iter().map(|(id, tile)| (id, tile.into_inner())).collect_vec();

    let tile_size = tiles[0].1.data[0].len();

    let tiles: HashMap<_, _> = tiles.into_iter().collect();
    let mut image: Vec<Vec<bool>> = vec![vec![]; (tile_size - 2) * size];

    for (i, row) in board.iter().enumerate() {
        for (j, id) in row.iter().enumerate() {
            let tile = tiles.get(&id).unwrap();

            for (ii, row_tile) in tile.data[1..tile_size-1].iter().enumerate() {
                for x in &row_tile[1..tile_size-1] {
                    image[i * (tile_size-2) + ii].push(*x);
                }
            }
        }
    }

    let mut image = Tile::new(0, image);

    let monster = vec![
        "                  # ".chars().map(|c| c == '#').collect_vec(),
        "#    ##    ##    ###".chars().map(|c| c == '#').collect_vec(),
        " #  #  #  #  #  #   ".chars().map(|c| c == '#').collect_vec(),
    ];

    let mut found = 0;
    let mut ignored_total = 0;
    'outer: for _ in 0..2 {
        image.flip();
        for _ in 0..4 {
            image.rotate();
            let num_rows = image.data.len();
            let num_cols = image.data[0].len();

            // Iterate over all possible starting points
            for orig_i in 0..(num_rows - monster.len() + 1) {
                'origin: for orig_j in 0..(num_cols - monster[0].len() + 1) {
                    let mut ignored = 0;
                    for (i, row) in monster.iter().enumerate() {
                        for (j, x) in row.iter().enumerate() {
                            if *x && !image.data[orig_i + i][orig_j + j] {
                                continue 'origin;
                            }
                            if *x {
                                ignored += 1;
                            }
                        }
                    }
                    found += 1;
                    ignored_total += ignored;
                }
            }

            if found > 0 {
                break 'outer;
            }
        }
    }

    let mut total_x = 0;
    for row in image.data {
        for x in row {
            if x {
                total_x += 1;
            }
        }
    }
    
    println!("part2={}", total_x - ignored_total);
}
