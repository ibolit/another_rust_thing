use another_rust_thing::api::Report;
use another_rust_thing::{devices, locations};


fn main() {
    // use crate::hello;
    
    let socket = devices::PlugSocket::new_grid_socket("Living room socket".to_owned());
    let socket = socket.switch_on();
    println!(
        "Socket nominal voltage is {}, but actual voltage is {}",
        socket.nomial_voltage,
        socket.get_actual_voltage()
    );
    println!("{}", socket.report());
    let socket = socket.switch_off();
    let thermometer = devices::Thermometer::new("Thermometer #1".to_owned());
    println!(
        "The current temperature is: {}",
        thermometer.temperature.celcius()
    );

    let mut house = locations::House {
        name: "My House".to_owned(),
        rooms: vec![],
    };

    let room = locations::Room {
        name: "My Room".to_owned(),
        devices: vec![&thermometer, &socket],
    };

    let mut room2 = locations::Room::new("my other room".to_owned());
    room2.add_device(&thermometer);
    room2.add_device(&socket);
    let thermo2 = devices::Thermometer::new("Thermometer #1".to_owned());
    let thermo3 = devices::Thermometer::new("Thermo 3".to_owned());
    println!("Number of devices in the room is: {}", room2.devices.len());
    room2.add_device(&thermo2);
    println!("Number of devices in the room is: {}", room2.devices.len());
    room2.add_device(&thermo3);
    println!("Number of devices in the room is: {}", room2.devices.len());

    println!("{:?}", room.name);

    house.add_room(&room);
    house.add_room(&room2);
    println!("Number of rooms in the house is: {}", house.rooms.len());

    println!("{}", house.report());
}
