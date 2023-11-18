#![no_std]
#![no_main]

use lockbsp::entry;
use lockbsp::hal as hal;
use locklogic::Component;
use panic_halt as _;

use lockbsp as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio,
    pac,
    spi,
    sio::Sio,
    watchdog::Watchdog,
};

use embedded_hal::digital::v2::OutputPin;
use fugit::RateExtU32;
use ssd1306::mode::DisplayConfig;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let oled_mosi = pins.oled_mosi.into_function::<gpio::FunctionSpi>();
    let oled_sck = pins.oled_sck.into_function::<gpio::FunctionSpi>();
    let oled_dc = pins.oled_dc.into_push_pull_output();
    let oled_ncs = pins.oled_ncs.into_push_pull_output();
    let mut oled_nrst = pins.oled_nrst.into_push_pull_output();

    oled_nrst.set_low().unwrap();
    delay.delay_ms(100);
    oled_nrst.set_high().unwrap();
    delay.delay_ms(100);

    let oled_spi = spi::Spi::<_, _, _, 8>::new(pac.SPI1, (oled_mosi, oled_sck))
        .init(&mut pac.RESETS, 125_000_000u32.Hz(), 16_000_000u32.Hz(), embedded_hal::spi::MODE_0);
    let oled_interface = display_interface_spi::SPIInterface::new(oled_spi, oled_dc, oled_ncs);
    let mut display = ssd1306::Ssd1306::new(oled_interface, ssd1306::size::DisplaySize128x64, ssd1306::rotation::DisplayRotation::Rotate0).into_buffered_graphics_mode();
    display.init().unwrap();
    let mut display = locklogic::display::Controller::new(display);

    let mut led_pin = pins.dbg.into_push_pull_output();

    loop {
        let inst = timer.get_counter();
        let us = inst.ticks();
        display.tick(us);

        // Demo mode.
        let s = (us as f32) / 1000_000.0;
        match (s as u32) % 8 {
            0 => { display.set_state(locklogic::display::State::Idle); },
            1 => { display.set_state(locklogic::display::State::PIN { digits: 0 }); },
            2 => { display.set_state(locklogic::display::State::PIN { digits: 1 }); },
            3 => { display.set_state(locklogic::display::State::PIN { digits: 2 }); },
            4 => { display.set_state(locklogic::display::State::PIN { digits: 3 }); },
            5 => { display.set_state(locklogic::display::State::PIN { digits: 4 }); },
            6 => { display.set_state(locklogic::display::State::Wrong); },
            7 => { display.set_state(locklogic::display::State::Correct); },
            _ => {},
        };

        led_pin.set_state(((s as u64) % 2 == 0).into()).ok();
    }
}
