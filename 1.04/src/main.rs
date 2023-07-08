use another_rust_thing::api::Device;
use another_rust_thing::{devices, house, locations, locations::House};

use another_rust_thing::devices::{PlugSocket, Thermometer};

use another_rust_thing::p2::locations as p2_locs;
use another_rust_thing::p2::{self, DeviceStorage};

fn main() {
    let mut sockets = p2::DeviceStorage::<PlugSocket>::new();
    let uuid1 = sockets.add(PlugSocket::new_grid_socket("one"));
    let uuid2 = sockets.add(PlugSocket::new_grid_socket("two"));

    let mut house = p2_locs::House::new("house of the rising sun");
    house.devices.push(uuid1);
    house.devices.push(uuid2);

    let mut thermos = p2::DeviceStorage::<Thermometer>::new();
    let thermo_uuid = thermos.add(Thermometer::new("thermo name"));
    house.devices.push(thermo_uuid);

    // let my_other_sockets = sockets as DeviceStorage<&dyn Device>;

    let my_plug_socket = PlugSocket::new_grid_socket("dude");
    let mps_ptr = &my_plug_socket as &dyn Device;
    let my_sockets = &sockets as &DeviceStorage<&dyn Device>;

    // let socket_reports = house.devices.iter().filter(|x| sockets.contains_uuid(**x));

    let house_report = house.report(&thermos);

    let my_vec: Vec<&DeviceStorage<&dyn Device>> = vec![&sockets, &thermos];
    let house_multi_report = house.multi_report(my_vec);

    println!("sr is {house_multi_report}")
}

fn main_old() {
    // use crate::hello;

    let socket = devices::PlugSocket::new_grid_socket("Living room socket");
    let socket = socket.switch_on();
    println!(
        "Socket nominal voltage is {}, but actual voltage is {}",
        socket.nomial_voltage,
        socket.get_actual_voltage()
    );
    // println!("{}", socket.report());
    let socket = socket.switch_off();
    let thermometer = devices::Thermometer::new("Thermometer #1");
    println!(
        "The current temperature is: {}",
        thermometer.temperature.celcius()
    );

    let room = locations::Room {
        name: "My Room".to_owned(),
        devices: vec![&thermometer, &socket],
    };

    let mut room2 = locations::Room::new("my other room");
    room2.add_device(&thermometer);
    room2.add_device(&socket);
    let thermo2 = devices::Thermometer::new("Thermometer #1");
    let thermo3 = devices::Thermometer::new("Thermo 3");
    println!("Number of devices in the room is: {}", room2.devices.len());
    room2.add_device(&thermo2);
    println!("Number of devices in the room is: {}", room2.devices.len());
    room2.add_device(&thermo3);
    println!("Number of devices in the room is: {}", room2.devices.len());

    println!("{:?}", room.name);

    let house = house!(name: "My House", rooms: &room, &room2);
    println!("Number of rooms in the house is: {}", house.rooms.len());

    // println!("{}", house.report());
}
