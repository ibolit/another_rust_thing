use crate::api::{Device, Named, Report};

pub trait ReportNamedDevice: Named + Report + Device {}
impl<T: Named + Report + Device> ReportNamedDevice for T {}

pub struct Room<'a> {
    pub name: String,
    pub devices: Vec<&'a dyn ReportNamedDevice>,
}

impl<'a> Named for Room<'a> {
    fn name(&self) -> &String {
        &self.name
    }
}
impl<'a> Report for Room<'a> {
    fn report(&self) -> String {
        let mut ret = format!("Room {}\n", self.name());
        for device in self.devices.iter() {
            ret += device.report().as_str();
        }
        ret + "\n"
    }
}

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
    pub fn add_room(&mut self, room: &'a Room) {
        if !name_is_in_named_vector(room.name(), &self.rooms) {
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

impl<'a> Room<'a> {
    pub fn new(name: String) -> Self {
        Room {
            name,
            devices: vec![],
        }
    }
}

impl<'a> Room<'a> {
    pub fn add_device(&mut self, device: &'a dyn ReportNamedDevice) {
        if !name_is_in_named_vector(device.name(), &self.devices) {
            self.devices.push(device)
        }
    }
}

fn name_is_in_named_vector<T: ?Sized + Named>(name: &str, haystack: &[&T]) -> bool {
    haystack
        .iter()
        .map(|x| x.name().clone())
        .collect::<Vec<String>>()
        .contains(&name.to_string())
}
