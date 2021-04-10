use std::collections::HashMap;

#[derive(Debug)]
pub struct Room {
    players: std::vec::Vec<usize>,
    turn: usize,
    field: HashMap<(i32, i32), usize>,
}
