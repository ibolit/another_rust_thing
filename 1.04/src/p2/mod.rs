pub mod locations;
use std::collections::HashMap;

use uuid;

use crate::api::Device;

#[derive(Default)]
pub struct DeviceStorage<T: Device> {
    pub devices: HashMap<uuid::Uuid, T>,
    pub devices_by_name: HashMap<String, uuid::Uuid>,
}

impl<T: Device> DeviceStorage<T> {
    pub fn new() -> Self {
        DeviceStorage {
            devices: HashMap::new(),
            devices_by_name: HashMap::default(),
        }
    }

    pub fn add(&mut self, device: T) -> uuid::Uuid {
        let s = uuid::Uuid::new_v4();
        self.devices_by_name.insert(device.name().clone(), s);
        self.devices.insert(s, device);
        s
    }

    pub fn contains_uuid(&self, x: uuid::Uuid) -> bool {
        self.devices.contains_key(&x)
    }

    // pub fn another_by_uuids<'a>(
    //     devices: &'a DeviceStorage<impl ReportNamedDevice>,
    //     uuids: &[&uuid::Uuid],
    // ) -> impl Iterator<Item = &'a impl ReportNamedDevice> {
    //     devices
    //         .devices
    //         .iter()
    //         .filter(move |(uuid, _)| uuids.contains(uuid))
    //         .map(|(_, socket)| socket)
    // }

    // pub fn by_uuids<'a>(devices: &'a DeviceStorage<T>, uuids: &[&uuid::Uuid]) -> Vec<&'a T> {
    //     devices
    //         .devices
    //         .iter()
    //         .filter(move |(uuid, _)| uuids.contains(uuid))
    //         .map(|(_, socket)| socket)
    //         .collect::<Vec<&'a T>>()
    // }
    pub fn by_uuids(&self, uuids: &[uuid::Uuid]) -> Vec<&T> {
        self.devices
            .iter()
            .filter(move |(uuid, _)| uuids.contains(uuid))
            .map(|(_, socket)| socket)
            .collect::<Vec<&T>>()
    }
}
