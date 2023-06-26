mod devices;

fn main() {
    let socket = devices::PlugSocket::new_grid_socket("Living room socket".to_owned());
    let socket = socket.switch_on();
    println!(
        "Socket nominal voltage is {}, but actual voltage is {}",
        socket.nomial_voltage,
        socket.get_actual_voltage()
    );
    socket.switch_off();
    let thermometer = devices::Thermometer::new();
    println!(
        "The current temperature is: {}",
        thermometer.temperature.celcius()
    )
}
