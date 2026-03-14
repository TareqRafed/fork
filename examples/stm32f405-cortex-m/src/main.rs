#![no_std]
#![no_main]

extern crate panic_halt;

use core::{
    cell::{Cell, RefCell},
    ops::DerefMut,
    sync::atomic::Ordering,
};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    gpio::Edge,
    pac::{self, interrupt},
    prelude::*,
    rcc::Config,
    rcc::Rcc,
    timer::{CounterUs, Event, Flag, Timer},
};

use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};

use rtt_target::{rprintln, rtt_init_print};

static TIMER_TIM2: Mutex<RefCell<Option<CounterUs<pac::TIM2>>>> = Mutex::new(RefCell::new(None));

static ELAPSED_MS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0u32));

use crate::{
    buttons::{LEFT_BUTTON, LEFT_BUTTON_FLAG, RIGHT_BUTTON, RIGHT_BUTTON_FLAG},
    buzzer::Buzzer,
    player::App,
};

mod buttons;
mod buzzer;
mod player;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut rcc = setup_clocks(dp.RCC);

    let mut syscfg = dp.SYSCFG.constrain(&mut rcc);

    let mut delay = Timer::syst(cp.SYST, &rcc.clocks).delay();

    // GPIO
    let gpiob = dp.GPIOB.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);
    let gpioa = dp.GPIOA.split(&mut rcc);

    // LCD
    let rs = gpioc.pc2.into_push_pull_output();
    let en = gpioc.pc3.into_push_pull_output();
    let d4 = gpiob.pb9.into_push_pull_output();
    let d5 = gpiob.pb8.into_push_pull_output();
    let d6 = gpioc.pc6.into_push_pull_output();
    let d7 = gpioc.pc7.into_push_pull_output();
    rprintln!("init lcd!");

    let mut lcd = HD44780::new_4bit(rs, en, d4, d5, d6, d7, &mut delay).unwrap();
    lcd.reset(&mut delay).unwrap();
    lcd.clear(&mut delay).unwrap();
    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Invisible,
            cursor_blink: CursorBlink::Off,
        },
        &mut delay,
    )
    .unwrap();
    rprintln!("init buzzer!");

    // Buzzer
    let (pwm_mngr, (pwm_ch, ..)) = dp.TIM3.pwm_hz(20.kHz(), &mut rcc);

    let mut pwm_c1 = pwm_ch.with(gpioa.pa6);

    rprintln!("init tim2!");

    let mut timer = dp.TIM2.counter_us(&mut rcc);
    timer.start(1.millis()).unwrap();
    timer.listen(Event::Update);

    let buzzer = Buzzer::new(pwm_mngr, pwm_c1);

    rprintln!("init buttons!");

    // Buttons
    let mut l_btn = gpioa.pa4;
    l_btn.make_interrupt_source(&mut syscfg);
    l_btn.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    l_btn.enable_interrupt(&mut dp.EXTI);

    let mut r_btn = gpioa.pa5;
    r_btn.make_interrupt_source(&mut syscfg);
    r_btn.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    r_btn.enable_interrupt(&mut dp.EXTI);

    unsafe {
        cortex_m::peripheral::NVIC::unmask(r_btn.interrupt());
        cortex_m::peripheral::NVIC::unmask(l_btn.interrupt());
        cortex_m::peripheral::NVIC::unmask(interrupt::TIM2);
    }

    cortex_m::interrupt::free(|cs| {
        RIGHT_BUTTON.borrow(cs).replace(Some(r_btn));
        LEFT_BUTTON.borrow(cs).replace(Some(l_btn));
        TIMER_TIM2.borrow(cs).replace(Some(timer));
    });

    rprintln!("init player!");
    let mut player = App::new(lcd, buzzer, delay);
    rprintln!("start the loop!");

    loop {
        player.tick();
        if LEFT_BUTTON_FLAG.swap(false, Ordering::Relaxed) {
            rprintln!("Left is clicked!");
            player.prev_song();
        }

        if RIGHT_BUTTON_FLAG.swap(false, Ordering::Relaxed) {
            rprintln!("Right is clicked!");
            player.next_song();
        }
    }
}

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut tim2) = TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
            tim2.clear_flags(Flag::Update);
        }
        let cell = ELAPSED_MS.borrow(cs);
        let val = cell.get();
        cell.replace(val + 1);
        rprintln!("MS {:?}", val);
    });
}

#[interrupt]
fn EXTI4() {
    LEFT_BUTTON_FLAG.swap(true, Ordering::Relaxed);
    cortex_m::interrupt::free(|cs| {
        let mut button = LEFT_BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });
}

#[interrupt]
fn EXTI9_5() {
    RIGHT_BUTTON_FLAG.swap(true, Ordering::Relaxed);
    cortex_m::interrupt::free(|cs| {
        let mut button = RIGHT_BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });
}

fn setup_clocks(rcc: pac::RCC) -> Rcc {
    rcc.freeze(
        Config::hsi()
            .hclk(48.MHz())
            .sysclk(48.MHz())
            .pclk1(24.MHz())
            .pclk2(24.MHz()),
    )
}
