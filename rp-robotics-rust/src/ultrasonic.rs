use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

const GPIO_TRIGGER: u8 = 17;
const GPIO_ECHO: u8 = 27;

pub fn launch() -> Result<(), Box<dyn Error>> {
    let mut trigger_pin = Gpio::new()?.get(GPIO_TRIGGER)?.into_output();
    let mut echo_pin = Gpio::new()?.get(GPIO_ECHO)?.into_output();

    loop {
        println!(
            "Distance: {} cm",
            get_distance(&mut trigger_pin, &mut echo_pin)
        );

        thread::sleep(Duration::from_millis(1000));

    }
}

fn get_distance(trigger_pin: &mut OutputPin, echo_pin: &mut OutputPin) -> f32 {
    trigger_pin.set_high();
    thread::sleep(Duration::from_millis(1));
    echo_pin.set_low();
    let mut start_time: Instant = Instant::now();
    let mut duration: Duration = start_time.elapsed();
    while echo_pin.is_set_low() {
        start_time = Instant::now();
    }
    while echo_pin.is_set_high() {
        duration = start_time.elapsed();
    }
    duration.as_secs_f32() * 34300.0 / 2.0
}
