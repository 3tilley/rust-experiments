use wasapi::{Device, DeviceCollection, Direction, get_default_device, initialize_mta};

fn main() {
    let _ = initialize_mta();

    let default_device = get_default_device(&Direction::Render).unwrap();
    println!("Default device: {}", default_device.get_friendlyname().unwrap());

    println!("Fetching playback devices...");
    enumerate_devices();

    println!("Checking headset and speakers...");
    let (headset, speakers) = check_headset_and_speakers();
    if let Some(headset) = headset {
        println!("Headset: {}", headset.get_friendlyname().unwrap());
    } else {
        println!("Headset not found");
    }
    if let Some(speakers) = speakers {
        println!("Speakers: {}", speakers.get_friendlyname().unwrap());
    } else {
        println!("Speakers not found");
    }

}

pub struct DeviceCollectionIter {
    collection: DeviceCollection,
    index: usize,
}

impl DeviceCollectionIter {
    pub fn new(collection: DeviceCollection) -> Self {
        Self {
            collection,
            index: 0,
        }
    }
}

impl Iterator for DeviceCollectionIter {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.collection.get_nbr_devices().unwrap() as usize {
            return None;
        }
        let device = self.collection.get_device_at_index(self.index as u32).unwrap();
        self.index += 1;
        Some(device)
    }
}

pub fn check_headset_and_speakers() -> (Option<Device>, Option<Device>) {
    let devices = DeviceCollection::new(&Direction::Render).unwrap();
    let mut headset = None;
    let mut speakers = None;
    for device in DeviceCollectionIter::new(devices) {
        if device.get_friendlyname().unwrap().contains("MPOW Wireless Gaming Headset") {
            headset = Some(device);
        } else if device.get_friendlyname().unwrap().contains("FrontAudioJack") {
            speakers = Some(device);
        }
    }
    (headset, speakers)
}

// Enumerate devices using DeviceCollection
pub fn enumerate_devices() {
    let devices = DeviceCollection::new(&Direction::Render).unwrap();
    for device in DeviceCollectionIter::new(devices) {
        println!("Device: {}. State: {}", device.get_friendlyname().unwrap(), device.get_state().unwrap());
    }
    // let num = devices.get_nbr_devices().unwrap();
    // for i in 0..num {
    //     println!("Device: {}", devices.get_device_at_index(i).unwrap().get_friendlyname().unwrap());// name().unwrap());
    // }
}