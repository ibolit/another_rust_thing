use core::result::Result;

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
    pub fn new_grid_socket(description: String) -> Self {
        PlugSocket {
            description,
            nomial_voltage: 220.0,
            variant: Current::AC { phase: 120.0 },
            actual_voltage: 0.0,
            actual_current: 0.0,
            state: std::marker::PhantomData::<Off>,
        }
    }
}

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

    pub fn _get_power_consumption() -> Option<f64> {
        todo!("Find the formula for getting the wattage depending on the phase, voltae and current")
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
    pub temperature: Temperature,
}

impl Thermometer {
    pub fn new() -> Self {
        Thermometer {
            temperature: Temperature::new(293.0).unwrap(),
        }
    }
}
