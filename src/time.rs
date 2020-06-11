use std::time::{SystemTime, Duration};

#[derive(PartialEq)]
pub enum State {
    Idle,
    Active,
    Inactive,
}

pub struct Timer {
    pub state: State,
    time: SystemTime,
    result: Duration,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            state: State::Inactive,
            time: SystemTime::now(),
            result: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) {
        self.time = SystemTime::now();
        self.state = State::Active;
    }

    pub fn stop(&mut self) {
        self.result = self.time.elapsed().unwrap();
        self.state = State::Inactive;
    }

    pub fn idle(&mut self) {
        self.state = State::Idle;
    }
}

use std::fmt;

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let current = if self.state == State::Active {
            self.time.elapsed().unwrap()
        } else {
            self.result
        }.as_millis();

        write!(f, "{}.{}", current / 1000, current % 1000)
    }
}
