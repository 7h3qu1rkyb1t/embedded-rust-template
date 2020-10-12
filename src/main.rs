#![no_main]
#![no_std]

use panic_halt as _; // panic handler
use rtic::app;
use stm32f7::stm32f7x7;

#[app(device = stm32f7x7)]
const APP: () = {};
