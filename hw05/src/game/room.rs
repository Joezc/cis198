use std::rc::Rc;
use std::cell::RefCell;
//use std::mem;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {
    // TODO: Implement the necessary methods for Rooms.
    pub fn new() -> Room {
        Room {
            name : "name".to_string(),
            contents: vec![],
            halls: vec![],
            wumpus: false
        }
    }

    pub fn neighbors(&self) -> Vec<Rc<RefCell<Room>>> {
        let mut ans = vec![];
        for hall in self.halls.clone() {
            ans.push(hall.other(&self));
        }
        ans
    }

    pub fn neighbors_string(&self) -> String {
        self.neighbors().iter().map(|n| {
            let n = n.clone();
            let tmp = n.borrow();
            tmp.name.to_lowercase()
        }).collect::<Vec<String>>().join(", ")
    }
}
