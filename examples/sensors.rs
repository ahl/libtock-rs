#![no_std]

use core::fmt::Write;
use libtock::result::TockResult;
use libtock::sensors::Ninedof;
use libtock::sensors::*;
use libtock::timer;
use libtock::timer::Duration;
use libtock::Hardware;

#[libtock::main]
async fn main() -> TockResult<()> {
    let Hardware { console_driver } = libtock::retrieve_hardware()?;
    let mut console = console_driver.create_console();
    let mut humidity = HumiditySensor;
    let mut temperature = TemperatureSensor;
    let mut light = AmbientLightSensor;
    let mut ninedof = Ninedof::default();
    let context = timer::DriverContext::create()?;
    let mut driver = context.create_timer_driver()?;
    let timer_driver = driver.activate()?;

    loop {
        writeln!(console, "Humidity:    {}\n", humidity.read()?)?;
        writeln!(console, "Temperature: {}\n", temperature.read()?)?;
        writeln!(console, "Light:       {}\n", light.read()?)?;
        writeln!(console, "Accel:       {}\n", ninedof.read_acceleration()?)?;
        timer_driver.sleep(Duration::from_ms(500)).await?;
    }
}
