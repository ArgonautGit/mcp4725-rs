#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Speed},
    i2c::{self, Config},
    time::Hertz,
};
use embassy_time::Timer;
use micromath::F32Ext;
use {defmt_rtt as _, panic_probe as _};

use mcp4725::{Dac, DacAsync, Mcp4725};

bind_interrupts!(struct Irqs {
    I2C1 => embassy_stm32::i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C1>,
           embassy_stm32::i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let led = Output::new(p.PA5, Level::High, Speed::Low);
    // let mut i2c = i2c::I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz::khz(100), Config::default());
    let mut i2c = i2c::I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        Hertz::khz(100),
        Config::default(),
    );

    spawner.spawn(blinky(led)).unwrap();

    let mut mcp = Mcp4725::new(0x60, 3.33);
    // mcp.set_voltage(&mut i2c, 1.45).expect("FAILED!!!!!");

    // i2c.write(0x69, &[0x1]).unwrap();

    // let bus = Mutex::new(RefCell::new(i2c));
    // let bus_mcp = BusMcp4725::new(CriticalSectionDevice::new(&bus), 0x60, 3.33);

    // let mcp1 = Mcp4725::new(0x60, supply_voltage)

    let mut x: f32 = 0.0;
    loop {
        mcp.set_voltage(&mut i2c, x.sin() * 1.65 + 1.65)
            .await
            .unwrap();
        x += 0.1;
    }
}

#[embassy_executor::task]
async fn blinky(mut led: Output<'static>) {
    loop {
        led.toggle();
        info!("toggling");
        Timer::after_secs(1).await;
    }
}
