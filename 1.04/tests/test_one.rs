use another_rust_thing::api::Report;
use another_rust_thing::devices::{PlugSocket, Thermometer};
use another_rust_thing::house;
use another_rust_thing::locations::{House, Room};

#[test]
fn test_house_report() {
    let mut kitchen = Room::new("Kitchen");
    let mut bedroom = Room::new("Bedroom");
    let mut house = House::new("Baskerville Hall");
    let kettle_socket = PlugSocket::new_grid_socket("kettle socket");
    let oven_thermometer = Thermometer::new("Oven thermometer");

    let reading_lamp_socket = PlugSocket::new_grid_socket("Reading lamp socket");
    let bedroom_thermometer = Thermometer::new("Bedroom thermometer");
    kitchen.add_device(&kettle_socket);
    kitchen.add_device(&oven_thermometer);

    bedroom.add_device(&reading_lamp_socket);
    bedroom.add_device(&bedroom_thermometer);

    house.add_room(&kitchen);
    house.add_room(&bedroom);

    let report = house.report();
    assert_eq!(
        report,
        "House Baskerville Hall\nRoom Kitchen\nPlugSocket: \n    name: kettle socket\n    state: Off\nThermometer:\n    name: Oven thermometer\n    temperature: 19.850\n\nRoom Bedroom\nPlugSocket: \n    name: Reading lamp socket\n    state: Off\nThermometer:\n    name: Bedroom thermometer\n    temperature: 19.850\n\n\n"
    );
}

#[test]
fn test_house_macro() {
    let kitchen = Room::new("Kitchen");
    let bedroom = Room::new("Bedroom");

    let house = house!(name: "Macro house", rooms: &kitchen, &bedroom);
    assert_eq!(house.rooms.len(), 2);
    assert_eq!(
        house.report(),
        "House Macro house\nRoom Kitchen\n\nRoom Bedroom\n\n\n"
    );
}
