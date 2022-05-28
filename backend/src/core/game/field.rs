pub type Coordinates = (i32, i32);

use std::collections::HashMap;

pub struct EndlessField<F: Eq> {
    cells: HashMap<Coordinates, F>,
}

impl<FigureType: Eq> EndlessField<FigureType> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_figure(&mut self, figure: FigureType, coord: Coordinates) -> Result<(), String> {
        if self.cells.contains_key(&coord) {
            Err(String::from("Cell is taken"))
        } else {
            self.cells.insert(coord, figure);
            Ok(())
        }
    }
}

impl<F: Eq> Default for EndlessField<F> {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }
}

pub enum ClassicFigureType {
    Nought,
    Cross,
}

impl PartialEq for ClassicFigureType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nought, Self::Nought) => true,
            (Self::Cross, Self::Cross) => true,
            _ => false,
        }
    }
}