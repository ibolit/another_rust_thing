use crate::api::{Device, Named, Report};

pub trait ReportNamedDevice: Named + Device {}
impl<T: Named + Device> ReportNamedDevice for T {}

pub struct Room<'a> {
    pub name: String,
    pub devices: Vec<&'a dyn ReportNamedDevice>,
}

impl<'a> Named for Room<'a> {
    fn name(&self) -> &String {
        &self.name
    }
}

pub struct House<'a> {
    pub name: String,
    pub rooms: Vec<&'a Room<'a>>,
}

impl<'a> House<'a> {
    pub fn add_room(&mut self, room: &'a Room) {
        if !name_is_in_named_vector(room.name(), &self.rooms) {
            self.rooms.push(room)
        }
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
