#![no_std]
#![no_main]

use core::cell::RefCell;

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;
use tm4c123x;
use tm4c123x::Interrupt;
use tm4c123x::interrupt;

const OFF: u32 = 0x00; // all LEDs off
const ON: u32 = 0x02; // red LED on

static GPIO_MUTEX: Mutex<RefCell<Option<tm4c123x::GPIO_PORTF>>> = Mutex::new(RefCell::new(None));

fn setupGPIO(){
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = tm4c123x::Peripherals::take().unwrap();
    //setup();
    p.SYSCTL.rcgcgpio.write(|w| unsafe{w.bits(0x01 << 5)});
    let sysctl = p.SYSCTL;
    // let test = &sysctl.rcgc2;
    //sysctl.rcgcgpio.write(|w| unsafe { w.bits(0x01 << 5) }); // clock for portf
    //sysctl.rcgcgpio.write(|w| w.r5().set_bit()); // clock for portf without unsafe
    //sysctl.rcgcgpio.modify(|r, w| unsafe{w.bits(r.bits() | (0x01 << 5))}); 
    let portf = p.GPIO_PORTF; // OWNERSHIP TRANSFER!
    //portf.lock.write(|w| unsafe{w.bits(0x4C4F434B)}); // unlock
    portf.lock.write(|w| w.lock().unlocked()); // unlock woithout unsafe
    portf.cr.write(|w| unsafe{w.bits(0x1F)});
    portf.dir.write(|w| unsafe{w.bits(0x0E)});
    portf.pur.write(|w| unsafe{w.bits(0x11)});
    portf.den.write(|w| unsafe{w.bits(0x1F)});
    // portf.data.write(|w| unsafe{w.bits(ON)});

    // interrupt setup
    portf.is.write(|w| unsafe {w.bits(0x00)});
    portf.ibe.write(|w| unsafe{w.bits(0x00)});
    portf.iev.write(|w| unsafe{w.bits(0x00)});
    portf.im.write(|w| unsafe{w.bits(0x11)}); // interrupt mask -> enable for pf0 and pf4

    // unsafe{cp.NVIC.iser[0].write(1 << 30)};
    // let mut nvic = cp.NVIC;
    // nvic.enable(Interrupt::GPIOF); //deprecated
    unsafe {cortex_m::peripheral::NVIC::unmask(Interrupt::GPIOF)}; // new version (is unsafe)

    // move the GPIO_PORTF object to the mutex (mutex takes ownership)
    cortex_m::interrupt::free(|cs| GPIO_MUTEX.borrow(cs).replace(Some(portf)));
}

#[entry]
fn main() -> ! {
    setupGPIO();
    loop {
        asm::nop();
    }
}

#[interrupt]
fn GPIOF() {
    static mut led_state: bool = false;
    *led_state = !*led_state;
    let value = if *led_state {ON} else {OFF};
    asm::nop();
    cortex_m::interrupt::free(|cs|{
        let mutex_res_portf = GPIO_MUTEX.borrow(cs).borrow();
        let portf = mutex_res_portf.as_ref().unwrap();
        portf.data.write(|w| unsafe{w.bits(value)});
        portf.icr.write(|w| unsafe{w.bits(0x11)});
    });
}
