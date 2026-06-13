// Copyright 2025 Junseop Weon

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

const YEAR_CYCLE: [i64; 4] = [365, 365, 366, 365];
const MONTH: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_MONTH: [i64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub fn now() -> std::time::Duration {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Clock may have gone backwards")
}

pub struct Date {
    years: u32,
    months: u8,
    days: u8,
    hours: u8,
    minus: u8,
    secs: u8,
    millis: u16,
}

impl Date {
    pub fn now(offset_hours: i64) -> Self {
        let mut millis = crate::now().as_millis() as i64 + offset_hours * 3600 * 1000;
        let mut years = 1970;
        let mut months = 1;
        let mut days = 1 + millis / (24 * 3600 * 1000);
        millis %= 24 * 3600 * 1000;
        // Hours:Minus:Secs.Millis
        let hours = millis / (3600 * 1000);
        millis %= 3600 * 1000;
        let minus = millis / (60 * 1000);
        millis %= 60 * 1000;
        let secs = millis / 1000;
        millis %= 1000;
        // Years-Months-Days
        let mut cycle = 0;
        while YEAR_CYCLE[cycle] < days {
            days -= YEAR_CYCLE[cycle];
            years += 1;
            cycle += 1;
            if 3 < cycle {
                cycle = 0;
            }
        }
        if cycle == 2 {
            // leap year
            cycle = 0;
            while LEAP_MONTH[cycle] < days {
                days -= LEAP_MONTH[cycle];
                months += 1;
                cycle += 1;
            }
        } else {
            cycle = 0;
            while MONTH[cycle] < days {
                days -= MONTH[cycle];
                months += 1;
                cycle += 1;
            }
        }
        Date { years, months, days: days as u8, hours: hours as u8, minus: minus as u8, secs: secs as u8, millis: millis as u16 }
    }

    pub fn secs(&self) -> u8 {
        self.secs
    }

    pub fn millis(&self) -> u16 {
        self.millis
    }
}

impl std::fmt::Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}", self.years, self.months, self.days, self.hours, self.minus, self.secs, self.millis)
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}