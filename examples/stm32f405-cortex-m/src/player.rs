use cortex_m::interrupt::free;
use hd44780_driver::{bus::DataBus, CursorBlink, HD44780};
use rtt_target::rprintln;
use stm32f4xx_hal::timer::SysDelay;

use crate::{buzzer::Buzzer, ELAPSED_MS};

const NOTE_B0: u32 = 31;
const NOTE_C1: u32 = 33;
const NOTE_CS1: u32 = 35;
const NOTE_D1: u32 = 37;
const NOTE_DS1: u32 = 39;
const NOTE_E1: u32 = 41;
const NOTE_F1: u32 = 44;
const NOTE_FS1: u32 = 46;
const NOTE_G1: u32 = 49;
const NOTE_GS1: u32 = 52;
const NOTE_A1: u32 = 55;
const NOTE_AS1: u32 = 58;
const NOTE_B1: u32 = 62;

const NOTE_C2: u32 = 65;
const NOTE_CS2: u32 = 69;
const NOTE_D2: u32 = 73;
const NOTE_DS2: u32 = 78;
const NOTE_E2: u32 = 82;
const NOTE_F2: u32 = 87;
const NOTE_FS2: u32 = 93;
const NOTE_G2: u32 = 98;
const NOTE_GS2: u32 = 104;
const NOTE_A2: u32 = 110;
const NOTE_AS2: u32 = 117;
const NOTE_B2: u32 = 123;

const NOTE_C3: u32 = 131;
const NOTE_CS3: u32 = 139;
const NOTE_D3: u32 = 147;
const NOTE_DS3: u32 = 156;
const NOTE_E3: u32 = 165;
const NOTE_F3: u32 = 175;
const NOTE_FS3: u32 = 185;
const NOTE_G3: u32 = 196;
const NOTE_GS3: u32 = 208;
const NOTE_A3: u32 = 220;
const NOTE_AS3: u32 = 233;
const NOTE_B3: u32 = 247;

const NOTE_C4: u32 = 262;
const NOTE_CS4: u32 = 277;
const NOTE_D4: u32 = 294;
const NOTE_DS4: u32 = 311;
const NOTE_E4: u32 = 330;
const NOTE_F4: u32 = 349;
const NOTE_FS4: u32 = 370;
const NOTE_G4: u32 = 392;
const NOTE_GS4: u32 = 415;
const NOTE_A4: u32 = 440;
const NOTE_AS4: u32 = 466;
const NOTE_B4: u32 = 494;

const NOTE_C5: u32 = 523;
const NOTE_CS5: u32 = 554;
const NOTE_D5: u32 = 587;
const NOTE_DS5: u32 = 622;
const NOTE_E5: u32 = 659;
const NOTE_F5: u32 = 698;
const NOTE_FS5: u32 = 740;
const NOTE_G5: u32 = 784;
const NOTE_GS5: u32 = 830;
const NOTE_A5: u32 = 880;
const NOTE_AS5: u32 = 932;
const NOTE_B5: u32 = 988;

const MARIO_THEME: &[(u32, u32)] = &[
    // Intro
    (NOTE_E5, 125),
    (NOTE_E5, 125),
    (0, 125),
    (NOTE_E5, 125),
    (0, 125),
    (NOTE_C5, 125),
    (NOTE_E5, 125),
    (0, 125),
    (NOTE_G5, 250),
    (0, 250),
    (NOTE_G4, 250),
    (0, 250),
    // First phrase
    (NOTE_C5, 250),
    (0, 125),
    (NOTE_G4, 250),
    (0, 125),
    (NOTE_E4, 250),
    (0, 125),
    (NOTE_A4, 250),
    (NOTE_B4, 125),
    (NOTE_AS4, 125),
    (NOTE_A4, 125),
    (NOTE_G4, 125),
    (NOTE_E5, 250),
    (NOTE_G5, 125),
    (NOTE_A5, 125),
    (NOTE_F5, 125),
    (NOTE_G5, 125),
    (0, 125),
    (NOTE_E5, 125),
    (NOTE_C5, 125),
    (NOTE_D5, 125),
    (NOTE_B4, 125),
    (0, 250),
    // Second phrase
    (NOTE_C5, 125),
    (0, 125),
    (NOTE_G4, 125),
    (NOTE_E4, 125),
    (0, 125),
    (NOTE_A4, 125),
    (NOTE_B4, 125),
    (0, 125),
    (NOTE_AS4, 125),
    (NOTE_A4, 125),
    (0, 125),
    (NOTE_G4, 125),
    (NOTE_E5, 125),
    (NOTE_G5, 125),
    (NOTE_A5, 125),
    (NOTE_F5, 125),
    (NOTE_G5, 125),
    (0, 125),
    (NOTE_E5, 125),
    (NOTE_C5, 125),
    (NOTE_D5, 125),
    (NOTE_B4, 125),
    (0, 250),
    // Main loop
    (NOTE_G5, 125),
    (NOTE_FS5, 125),
    (NOTE_F5, 125),
    (NOTE_DS5, 125),
    (NOTE_E5, 125),
    (0, 125),
    (NOTE_G5, 125),
    (NOTE_FS5, 125),
    (NOTE_F5, 125),
    (NOTE_DS5, 125),
    (NOTE_E5, 125),
    (0, 125),
    (NOTE_C5, 125),
    (NOTE_D5, 125),
    (NOTE_B4, 125),
    (0, 250),
    (NOTE_C5, 125),
    (NOTE_E5, 125),
    (NOTE_G5, 125),
    (0, 125),
    (NOTE_A5, 125),
    (NOTE_G5, 125),
    (NOTE_F5, 125),
    (NOTE_E5, 125),
    (0, 125),
    (NOTE_C5, 125),
    (NOTE_E5, 125),
    (NOTE_G5, 125),
    (0, 125),
    (NOTE_A5, 125),
    (NOTE_G5, 125),
    (NOTE_F5, 125),
    (NOTE_E5, 125),
    (0, 125),
];

const SEVEN_NATION_ARMY: &[(u32, u32)] = &[
    (NOTE_E3, 250),
    (NOTE_E3, 250),
    (NOTE_G3, 250),
    (NOTE_E3, 250),
    (NOTE_D3, 250),
    (NOTE_C3, 250),
    (NOTE_B2, 250),
    (0, 125),
    (NOTE_E3, 250),
    (NOTE_E3, 250),
    (NOTE_G3, 250),
    (NOTE_E3, 250),
    (NOTE_D3, 250),
    (NOTE_C3, 250),
    (NOTE_B2, 250),
    (0, 125),
    (NOTE_E3, 250),
    (NOTE_E3, 250),
    (NOTE_G3, 250),
    (NOTE_E3, 250),
    (NOTE_D3, 250),
    (NOTE_C3, 250),
    (NOTE_B2, 250),
    (0, 125),
    // Slight variation (higher octave)
    (NOTE_E4, 250),
    (NOTE_E4, 250),
    (NOTE_G4, 250),
    (NOTE_E4, 250),
    (NOTE_D4, 250),
    (NOTE_C4, 250),
    (NOTE_B3, 250),
    (0, 125),
    (NOTE_E4, 250),
    (NOTE_E4, 250),
    (NOTE_G4, 250),
    (NOTE_E4, 250),
    (NOTE_D4, 250),
    (NOTE_C4, 250),
    (NOTE_B3, 250),
    (0, 125),
    (NOTE_E3, 250),
    (NOTE_E3, 250),
    (NOTE_G3, 250),
    (NOTE_E3, 250),
    (NOTE_D3, 250),
    (NOTE_C3, 250),
    (NOTE_B2, 250),
    (0, 125),
    (NOTE_E3, 250),
    (NOTE_E3, 250),
    (NOTE_G3, 250),
    (NOTE_E3, 250),
    (NOTE_D3, 250),
    (NOTE_C3, 250),
    (NOTE_B2, 250),
    (0, 125),
];

pub struct Song {
    pub name: &'static str,
    pub data: &'static [(u32, u32)],
}

pub struct App<B: DataBus> {
    lcd: HD44780<B>,
    buzzer: Buzzer,
    current: usize,
    pub songs: &'static [Song],
    delay: SysDelay,
    current_song_time: u32,
}

impl<B: DataBus> App<B> {
    pub fn new(lcd: HD44780<B>, buzzer: Buzzer, delay: SysDelay) -> Self {
        const SONGS: &[Song] = &[
            Song {
                name: "MARIO",
                data: MARIO_THEME,
            },
            Song {
                name: "Seven Nation Army",
                data: SEVEN_NATION_ARMY,
            },
        ];
        Self {
            songs: SONGS,
            current: 0,
            lcd,
            buzzer,
            delay,
            current_song_time: 0,
        }
    }

    fn write_title(&mut self, title: &str) {
        self.lcd.clear(&mut self.delay).unwrap();
        self.lcd.set_cursor_blink(CursorBlink::Off, &mut self.delay);
        self.lcd.set_cursor_pos(0x0, &mut self.delay);
        self.lcd.write_str(title, &mut self.delay);
    }

    fn write_player(&mut self) {
        let elapsed_ms = free(|cs| ELAPSED_MS.borrow(cs).get());
        // ---- compute total song duration ----
        let mut total_ms = 0;
        for &(_, d) in self.current_song().data {
            total_ms += d;
        }

        // ---- elapsed time ----
        let e_sec = elapsed_ms / 1000;
        let e_min = e_sec / 60;
        let e_sec = e_sec % 60;

        // ---- total time ----
        let t_sec = total_ms / 1000;
        let t_min = t_sec / 60;
        let t_sec = t_sec % 60;

        // ---- write to LCD ----
        self.lcd.set_cursor_pos(0x40, &mut self.delay);

        // elapsed mm:ss
        self.lcd
            .write_char((b'0' + e_min as u8) as char, &mut self.delay);
        self.lcd.write_char(':', &mut self.delay);
        self.lcd
            .write_char((b'0' + (e_sec / 10) as u8) as char, &mut self.delay);
        self.lcd
            .write_char((b'0' + (e_sec % 10) as u8) as char, &mut self.delay);

        self.lcd.write_str(" / ", &mut self.delay);

        // total mm:ss
        self.lcd
            .write_char((b'0' + t_min as u8) as char, &mut self.delay);
        self.lcd.write_char(':', &mut self.delay);
        self.lcd
            .write_char((b'0' + (t_sec / 10) as u8) as char, &mut self.delay);
        self.lcd
            .write_char((b'0' + (t_sec % 10) as u8) as char, &mut self.delay);
    }

    fn apply_song(&mut self) {
        let current_song = self.current_song();
        self.write_title(current_song.name);
    }

    pub fn tick(&mut self) {
        let elapsed_ms = free(|cs| ELAPSED_MS.borrow(cs).get());
        rprintln!("{:?}", elapsed_ms);
        let mut accumulated = 0;
        let mut current_note = None;

        for &(hz, duration) in self.current_song().data {
            if elapsed_ms < accumulated + duration {
                current_note = Some(hz);
                break;
            }
            accumulated += duration;
        }

        if let Some(hz) = current_note {
            self.buzzer.play(hz);
        } else {
            // elapsed_ms exceeded total song duration
            self.next_song();
            free(|cs| ELAPSED_MS.borrow(cs).set(0));
        }
        self.write_player();
    }

    pub fn current_song(&self) -> &Song {
        &self.songs[self.current]
    }

    pub fn next_song(&mut self) {
        self.current = (self.current + 1) % self.songs.len();
        self.apply_song();
    }

    pub fn prev_song(&mut self) {
        self.current = if self.current == 0 {
            self.songs.len() - 1
        } else {
            self.current - 1
        };
        self.apply_song();
    }
}
