#![no_main]
#![no_std]

use panic_itm;

use cortex_m_rt::entry;

use stm32f3_discovery::{leds::Leds, stm32f3xx_hal, switch_hal};
use stm32f3xx_hal::prelude::*;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use cortex_m::interrupt;
use cortex_m::iprintln;
use cortex_m::peripheral::ITM;

// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     interrupt::disable();

//     let itm = unsafe { &mut *ITM::ptr() };
//     let stim = &mut itm.stim[0];

//     iprintln!(stim, "{}", info);

//     loop {
//         // add some side effect to prevent this from turning into a UDF instruction
//         // see rust-lang/rust#28728 for details
//         atomic::compiler_fence(Ordering::SeqCst);
//     }
// }

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
