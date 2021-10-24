use sdl2::audio::{ AudioCallback, AudioSpecDesired };
use std::time::Duration;
use sdl2::AudioSubsystem;

pub struct Speaker {
    // sound_timer: u8,
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for Speaker {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {

        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}


impl Speaker {
    pub fn new() -> Self {
        Speaker {
            // sound_timer: 0,
            phase_inc: 440.0,
            phase: 0.0,
            volume: 0.25,
        }
    }

    pub fn play_sound(&self, sub_system: &AudioSubsystem) {
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = sub_system.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            Speaker {
                phase_inc: self.phase_inc / spec.freq as f32,
                ..*self
            }
        }).unwrap();

        device.resume();
        std::thread::sleep(Duration::from_millis(2000));
    }
}




