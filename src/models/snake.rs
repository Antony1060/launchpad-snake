use std::borrow::Borrow;
use std::collections::LinkedList;

use crate::models::direction::Direction;
use crate::models::vector2i::Vector2i;
use crate::{utils, PAD_HEIGHT, PAD_WIDTH};

#[derive(Debug, Clone)]
pub struct Snake {
    root: Vector2i,
    parts: LinkedList<Vector2i>,
    direction: Direction,
    apple: Vector2i,
}

impl Snake {
    pub fn new_at(root: Vector2i) -> Self {
        let mut instance = Self {
            root: root.clone(),
            parts: {
                let mut list = LinkedList::new();
                list.push_back(root);
                list
            },
            direction: Direction::Right,
            apple: Vector2i { x: 0, y: 0 },
        };

        instance.apple = instance.find_new_apple();

        instance
    }

    pub fn reset(&mut self) {
        self.parts = {
            let mut list = LinkedList::new();
            list.push_back(self.root.clone());
            list
        };
        self.direction = Direction::Right;
        self.apple = self.find_new_apple();
    }

    pub fn tick(&mut self) {
        let new_head = self
            .parts
            .back()
            .expect("back should exist")
            .move_with(self.direction);

        if self.is_inside(&new_head) && self.parts.len() != 1 {
            self.reset();
            return;
        }

        if new_head.x == self.apple.x && new_head.y == self.apple.y {
            self.apple = self.find_new_apple();
        } else {
            self.parts.pop_front();
        }

        self.parts.push_back(new_head);
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if direction != self.direction.opposite() {
            self.direction = direction;
        }
    }

    pub fn is_inside<V: Borrow<Vector2i>>(&self, vector: V) -> bool {
        let vector = vector.borrow();
        self.parts
            .iter()
            .any(|part| part.x == vector.x && part.y == vector.y)
    }

    fn find_new_apple(&self) -> Vector2i {
        let mut vector;

        loop {
            let (x, y) = (
                utils::random::random_range(0, PAD_WIDTH),
                utils::random::random_range(0, PAD_HEIGHT),
            );

            vector = Vector2i { x, y };

            if !self.is_inside(&vector) {
                break;
            }
        }

        vector
    }

    pub fn get_apple(&self) -> Vector2i {
        self.apple.clone()
    }
}

impl IntoIterator for Snake {
    type Item = Vector2i;
    type IntoIter = std::collections::linked_list::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.parts.into_iter()
    }
}
