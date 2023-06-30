use super::ReportNamedDevice;
use crate::api::{Named, Report};
use crate::util;

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
        if !util::name_is_in_named_vector(device.name(), &self.devices) {
            self.devices.push(device)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Device;

    struct MockDevice {
        name: String,
    }

    impl Report for MockDevice {
        fn report(&self) -> String {
            "Hello".to_owned()
        }
    }
    impl super::Named for MockDevice {
        fn name(&self) -> &String {
            &self.name
        }
    }
    impl Device for MockDevice {}

    #[test]
    fn test_add_device() {
        let mut room = Room::new("Hello".to_owned());
        let mock_device = MockDevice {
            name: "Hello".to_owned(),
        };
        room.add_device(&mock_device);
        assert_eq!(room.devices.len(), 1);
        let another_mock_device = MockDevice {
            name: "Hello".to_owned(),
        };
        room.add_device(&another_mock_device);
        assert_eq!(room.devices.len(), 1);
        let third_mock_device = MockDevice {
            name: "Bye".to_owned(),
        };
        room.add_device(&third_mock_device);
        assert_eq!(room.devices.len(), 2);
    }
}
