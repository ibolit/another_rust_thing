use super::api::{Device, Named, Report};

pub trait ReportNamedDevice: Named + Report + Device {}
impl<T: Named + Report + Device> ReportNamedDevice for T {}

mod house;
mod room;
pub use house::House;
pub use room::Room;
