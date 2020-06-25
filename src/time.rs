/* ************************************************************************** */
/*                                                                            */
/*                                                            .               */
/*   time.rs                                                 / \              */
/*                                                          /   \             */
/*   By: charles <charles.cabergs@gmail.com>               /o  o \            */
/*                                                        /  v    \           */
/*   Created: 2020/06/25 13:24:24 by charles             /    _    \          */
/*   Updated: 2020/06/25 13:24:24 by charles            '-----------'         */
/*                                                                            */
/* ************************************************************************** */

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

        write!(f, "{:0>2}.{:0>3}", current / 1000, current % 1000)
    }
}
