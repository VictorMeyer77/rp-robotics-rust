use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;

const GPIO_LED: u8 = 23;

pub fn launch() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    loop {
        flashing(&mut pin);
    }
}

fn flashing(pin: &mut OutputPin) -> () {
    if pin.is_set_high() {
        pin.set_low();
    } else {
        pin.set_high();
    }
    thread::sleep(Duration::from_millis(1000));
}
