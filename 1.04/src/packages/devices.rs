use core::result::Result;

use crate::api::{Device, Named, Report};

pub enum State {
    On,
    Off,
}

pub struct On;
pub struct Off;

pub enum Current {
    AC { phase: f64 },
    _DC,
}

pub struct PlugSocket<State = Off> {
    pub description: String,
    pub nomial_voltage: f64,
    pub variant: Current,
    actual_voltage: f64,
    actual_current: f64,
    state: std::marker::PhantomData<State>,
}

impl PlugSocket {
    pub fn new_grid_socket(description: &str) -> Self {
        PlugSocket {
            description: description.to_owned(),
            nomial_voltage: 220.0,
            variant: Current::AC { phase: 120.0 },
            actual_voltage: 0.0,
            actual_current: 0.0,
            state: std::marker::PhantomData::<Off>,
        }
    }
}

// impl PlugSocket {
//     fn get_state(&self) -> String {
//         "Unknown State".to_owned()
//     }
// }

// impl PlugSocket<Off> {
//     fn get_state(&self) -> String {
//         "Off".to_owned()
//     }
// }

// impl PlugSocket<On> {
//     fn get_state(&self) -> String {
//         "On".to_owned()
//     }
// }

impl PlugSocket<Off> {
    pub fn switch_on(self) -> PlugSocket<On> {
        PlugSocket {
            description: self.description,
            nomial_voltage: self.nomial_voltage,
            variant: self.variant,
            actual_voltage: self.actual_voltage,
            actual_current: self.actual_current,
            state: std::marker::PhantomData::<On>,
        }
    }
}

impl PlugSocket<On> {
    pub fn switch_off(self) -> PlugSocket<Off> {
        PlugSocket {
            description: self.description,
            nomial_voltage: self.nomial_voltage,
            variant: self.variant,
            actual_voltage: self.actual_voltage,
            actual_current: self.actual_current,
            state: std::marker::PhantomData::<Off>,
        }
    }
    pub fn get_actual_voltage(&self) -> f64 {
        233.0
    }

    pub fn _get_actual_current(&self) -> f64 {
        7.3
    }

    pub fn get_power_consumption(&self) -> f64 {
        300.9
    }
}

impl Device for PlugSocket {}
impl Named for PlugSocket {
    fn name(&self) -> &String {
        &self.description
    }
}
impl<State> Report for PlugSocket<State> {
    fn report(&self) -> String {
        format!(
            "PlugSocket: \n    name: {}\n    state: {:?}\n    power consumption: {}\n",
            self.description, self.state, 0.0
        )
    }
}
pub struct Temperature {
    degrees_kelvin: f64,
}

#[derive(Debug)]
pub enum TemperatureError {
    TooLow,
}

impl Temperature {
    pub fn new(t: f64) -> Result<Self, TemperatureError> {
        if t < 0.0 {
            Err(TemperatureError::TooLow)
        } else {
            Ok(Temperature { degrees_kelvin: t })
        }
    }
    pub fn celcius(&self) -> f64 {
        self.degrees_kelvin - 273.15
    }
    pub fn _kelvin(&self) -> f64 {
        self.degrees_kelvin
    }
    pub fn _fahrenheit(&self) -> f64 {
        self.celcius() * 9.0 / 5.0 + 32.0
    }
}

pub struct Thermometer {
    pub name: String,
    pub temperature: Temperature,
}

impl Thermometer {
    pub fn new(name: &str) -> Self {
        Thermometer {
            name: name.to_owned(),
            temperature: Temperature::new(293.0).unwrap(),
        }
    }
}

impl Device for Thermometer {}
impl Named for Thermometer {
    fn name(&self) -> &String {
        &self.name
    }
}
impl Report for Thermometer {
    fn report(&self) -> String {
        format!(
            "Thermometer:\n    name: {}\n    temperature: {:.3}\n",
            self.name(),
            self.temperature.celcius()
        )
    }
}
