use std::{thread, time};
use gpio::{GpioIn, GpioOut, GpioValue};
use gpio::sysfs::{SysFsGpioInput, SysFsGpioOutput};


const GPIO_OUT: u16 = 11;


fn flashing() -> () {
    let mut gpio_input: SysFsGpioInput = gpio::sysfs::SysFsGpioInput::open(GPIO_OUT).unwrap();
    let mut gpio_output: SysFsGpioOutput = gpio::sysfs::SysFsGpioOutput::open(GPIO_OUT).unwrap();
    let state = gpio_input.read_value().unwrap();
    println!("{:?}", state);
    if state == GpioValue::High {
        gpio_output.set_value(GpioValue::Low).expect("could not set gpio");
    } else {
        gpio_output.set_value(GpioValue::High).expect("could not set gpio");
    }
    thread::sleep(time::Duration::from_millis(1000));

}

fn main() {
    loop {
        flashing()
    }
}