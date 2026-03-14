use stm32f4xx_hal::{
    pac::{self, TIM3},
    prelude::_fugit_RateExtU32,
    timer::{pwm, PwmChannel},
};

pub struct Buzzer {
    pwm_mngr: pwm::PwmHzManager<pac::TIM3>,
    pub pwm: PwmChannel<TIM3, 0>,
}

impl Buzzer {
    pub fn new(pwm_mngr: pwm::PwmHzManager<pac::TIM3>, pwm_c1: PwmChannel<pac::TIM3, 0>) -> Self {
        let mut pwm = pwm_c1;

        let max = pwm.get_max_duty();
        pwm.set_duty(max / 2);
        pwm.enable();

        Self { pwm_mngr, pwm }
    }

    pub fn play(&mut self, hz: u32) {
        self.pwm_mngr.set_period(hz.Hz());

        let max = self.pwm.get_max_duty();
        self.pwm.set_duty(max / 2);
    }
}
