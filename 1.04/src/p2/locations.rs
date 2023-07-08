use crate::api::Device;

use super::DeviceStorage;

pub struct House {
    pub name: String,
    pub devices: Vec<uuid::Uuid>,
}

impl House {
    pub fn new(name: &str) -> Self {
        House {
            name: name.to_owned(),
            devices: vec![],
        }
    }

    pub fn report(&self, devices: &DeviceStorage<impl Device>) -> String {
        let a = devices.by_uuids(&self.devices);
        let report = a.iter().map(|x| x.report()).collect::<String>();
        report
    }

    pub fn multi_report(&self, device_collections: &[DeviceStorage<impl Device>]) -> String {
        let mut result = "".to_owned();
        for device_collection in device_collections {
            result.push_str(self.report(device_collection).as_str())
        }
        result
    }
}
