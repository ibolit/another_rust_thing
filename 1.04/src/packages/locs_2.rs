use crate::api::Device;

pub struct Room<'a> {
    devices: Vec<&'a dyn Device>,
}

impl<'a> Room<'a> {
    pub fn devices(&self) -> &Vec<&'a dyn Device> {
        &self.devices
    }
}
