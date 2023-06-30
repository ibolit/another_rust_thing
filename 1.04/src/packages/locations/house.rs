use super::Room;
use crate::api::{Named, Report};
use crate::util;

pub struct House<'a> {
    pub name: String,
    pub rooms: Vec<&'a Room<'a>>,
}

impl<'a> Named for House<'a> {
    fn name(&self) -> &String {
        &self.name
    }
}

impl<'a> House<'a> {
    pub fn new(name: &str) -> Self {
        House {
            name: name.to_owned(),
            rooms: vec![],
        }
    }
    pub fn add_room(&mut self, room: &'a Room) {
        if !util::name_is_in_named_vector(room.name(), &self.rooms) {
            self.rooms.push(room)
        }
    }
}
impl<'a> Report for House<'a> {
    fn report(&self) -> String {
        let mut ret = format!("House {}\n", self.name());
        for room in self.rooms.iter() {
            ret += room.report().as_str();
        }
        ret + "\n"
    }
}

#[macro_export]
macro_rules! house {
    (name: $name: expr, rooms: $($room: expr),*) => {
        {
            let mut house = House::new($name);
            $(
                house.add_room($room);
            )*
            house
        }
    };
}
