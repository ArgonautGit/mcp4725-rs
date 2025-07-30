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
use embedded_hal::i2c::SevenBitAddress;
use micromath::F32Ext;

use {defmt_rtt as _, panic_probe as _};

#[allow(unused_imports)]
use mcp4725::{DacAsync, DacBlocking, Mcp4725};

bind_interrupts!(struct Irqs {
    I2C1 => embassy_stm32::i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C1>,
           embassy_stm32::i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    /* Uncomment if you want to use the blocking configuration */
    // let mut i2c = i2c::I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz::khz(100), Config::default());

    // Comment this line out if you end up using the blocking configuration.
    let i2c = i2c::I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        Hertz::khz(100),
        Config::default(),
    );

    let led = Output::new(p.PA5, Level::High, Speed::Low);
    spawner.spawn(blinky(led)).unwrap();

    const MCP_ADDRESS: SevenBitAddress = 0x60;
    const SUPPLY_VOLTAGE: f32 = 3.33;
    let mut mcp = Mcp4725::new(MCP_ADDRESS, SUPPLY_VOLTAGE, i2c);

    let mut x: f32 = 0.0;
    loop {
        mcp.set_voltage(x.sin() * 1.65 + 1.65).await.unwrap();
        mcp.get_voltage().await.unwrap();
        x += 0.2;
        Timer::after_millis(500).await;
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
