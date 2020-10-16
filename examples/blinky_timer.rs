#![no_main]
#![no_std]

// use cortex_m_semihosting::{debug, hprintln};
use panic_halt as _;
use rtic::app;
// use cortex_m_semihosting::hprintln;
use stm32f7xx_hal::prelude::*;
use types::*;

#[app(device = stm32f7xx_hal::device, peripherals=true)]
mod app{
    use super::*;
    #[resources]
    struct Resources
    {
        led2:Led,
        timer: stm32f7xx_hal::device::SYST,
    }
    #[init]
    fn init(cx: init::Context)->init::LateResources{
        let core: cortex_m::Peripherals = cx.core;
        let device:stm32f7xx_hal::device::Peripherals = cx.device;
        let gpiob = device.GPIOB.split();
        let mut timer = core.SYST;
        timer.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        timer.set_reload(1_000_000);
        timer.enable_counter();
        let led2 = gpiob.pb7.into_push_pull_output();
        init::LateResources{led2, timer}
    }
    #[idle]
    fn idle(_c : idle::Context)->!{
        loop{
            blink::spawn().unwrap();
        }
    }
    #[task(resources = [led2, timer])]
    fn blink(c:blink::Context){
        c.resources.led2.set_low().unwrap();
        while !c.resources.timer.has_wrapped(){}
        c.resources.led2.set_high().unwrap();
        while !c.resources.timer.has_wrapped(){}
    }

    extern "C"{
        fn SPI4();
    }
}


mod types{
    use stm32f7xx_hal::gpio::{Output, PushPull};

    pub type Led = stm32f7xx_hal::gpio::gpiob::PB7<Output<PushPull>>;
}
