use core::{cell::RefCell, sync::atomic::AtomicBool};

use cortex_m::interrupt::Mutex;
use stm32f4xx_hal::gpio::{self, Input};

pub static LEFT_BUTTON_FLAG: AtomicBool = AtomicBool::new(false);
pub static RIGHT_BUTTON_FLAG: AtomicBool = AtomicBool::new(false);

type ButtonLeft = gpio::PA4<Input>;
type ButtonRight = gpio::PA5<Input>;

pub static LEFT_BUTTON: Mutex<RefCell<Option<ButtonLeft>>> = Mutex::new(RefCell::new(None));
pub static RIGHT_BUTTON: Mutex<RefCell<Option<ButtonRight>>> = Mutex::new(RefCell::new(None));

pub static FLAGS: [&'static AtomicBool; 2] = [&RIGHT_BUTTON_FLAG, &LEFT_BUTTON_FLAG];
