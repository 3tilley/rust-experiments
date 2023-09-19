use wasapi::{DeviceCollection, Direction, initialize_mta};

fn main() {
    println!("Fetching playback devices...");
    let _ = initialize_mta();
    enumerate_devices();
}

// Enumerate devices using DeviceCollection
pub fn enumerate_devices() {
    let devices = DeviceCollection::new(&Direction::Render).unwrap();
    let num = devices.get_nbr_devices().unwrap();
    for i in 0..num {
        println!("Device: {}", devices.get_device_at_index(i).unwrap().get_friendlyname().unwrap());// name().unwrap());
    }
}