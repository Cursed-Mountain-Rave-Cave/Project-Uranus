mod field;

use crate::data_type::CircularQueue;
use field::EndlessField;
use field::Coordinates;

pub trait Game<FigureType: Eq + Clone> {
    fn make_move(&mut self, figure: FigureType, coord: Coordinates) -> Result<(), String>;
}

pub struct EndlessFieldGame<F: Eq + Clone> {
    field: EndlessField<F>,
    figures_queue: CircularQueue<F>,
}

impl<FigureType: Eq + Clone> EndlessFieldGame<FigureType> {
    pub fn new() -> Self {
        Self {
            field: EndlessField::default(),
            figures_queue: CircularQueue::default(),
        }
    }

    fn put_current_figure(&mut self, coord: Coordinates) -> Result<(), String> {
        self.field.add_figure(self.figures_queue.next().clone(), coord)
    }
}

impl<FigureType: Eq + Clone> Game<FigureType> for EndlessFieldGame<FigureType> {
    fn make_move(&mut self, figure: FigureType, coord: Coordinates) -> Result<(), String> {
        if *self.figures_queue.current() == figure {
            self.put_current_figure(coord)
        } else {
            Err(String::from("Cant make such move"))
        }
    }
}

impl<T: Eq + Clone> Default for EndlessFieldGame<T> {
    fn default() -> Self {
        Self {
            field: EndlessField::default(),
            figures_queue: CircularQueue::default(),
        }
    }
}