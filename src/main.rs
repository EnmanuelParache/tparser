use clap::Parser;
use regex::Regex;
use std::fmt::Display;
/// Simple program to parse time
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Time in the hh:mm:ss form to be parsed
    time: String,

    /// Output unit can be
    /// Hours (h),
    /// Minutes (m) or
    /// Seconds (s)
    #[arg(short, long, default_value_t = 'h')]
    unit: char,

    /// Decompose
    #[arg(short, long)]
    decompose: bool,
}

/// Trait to convert to different
/// time units
trait ToTimeUnits {
    fn to_hours(&self) -> f32;
    fn to_minutes(&self) -> f32;
    fn to_seconds(&self) -> f32;
}

/// Trait to convert from different
/// time units
trait FromTimeUnits {
    fn from_hours(&self) -> Time;
    fn from_minutes(&self) -> Time;
    fn from_seconds(&self) -> Time;
}

/// Time struct to hold
/// hour, minutes and seconds
#[derive(Debug, PartialEq)]
struct Time {
    hour: u16,
    minutes: u8,
    seconds: u8,
}

/// Time struct implementation
impl Time {
    fn new() -> Self {
        Time {
            hour: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    fn set_hour(&mut self, hour: u16) {
        self.hour = hour;
    }

    fn set_minutes(&mut self, minutes: u8) {
        self.minutes = minutes;
    }

    fn set_seconds(&mut self, seconds: u8) {
        self.seconds = seconds;
    }
}

/// ToTimeUnits implementation for time
impl ToTimeUnits for Time {
    fn to_hours(&self) -> f32 {
        f32::from(self.hour)
            + (f32::from(self.minutes) / 60.0)
            + (f32::from(self.seconds) / (60.0 * 60.0))
    }
    fn to_minutes(&self) -> f32 {
        (f32::from(self.hour) * 60.0)
            + (f32::from(self.minutes))
            + (f32::from(self.seconds) / (60.0))
    }
    fn to_seconds(&self) -> f32 {
        (f32::from(self.hour) * 60.0 * 60.0)
            + (f32::from(self.minutes) * 60.0)
            + (f32::from(self.seconds))
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minutes, self.seconds)
    }
}

//
// Trait to convert f32 into time struct
//
impl FromTimeUnits for f32 {
    fn from_hours(&self) -> Time {
        Time {
            hour: self.trunc() as u16,
            minutes: ((self * 60.0) % 60.0).trunc() as u8,
            seconds: ((self * 3600.0) % 60.0).trunc() as u8,
        }
    }

    fn from_minutes(&self) -> Time {
        Time {
            hour: (self / 60.0).trunc() as u16,
            minutes: (self % 60.0).trunc() as u8,
            seconds: ((self * 60.0) % 60.0).trunc() as u8,
        }
    }

    fn from_seconds(&self) -> Time {
        Time {
            hour: (self / 3600.0).trunc() as u16,
            minutes: ((self / 60.0) % 60.0).trunc() as u8,
            seconds: (self % 60.0).trunc() as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_set_hour() {
        let mut time = Time::new();
        time.set_hour(1);

        assert_eq!(time.hour, 1);
    }

    #[test]
    fn test_to_hours() {
        let mut time = Time::new();
        time.set_hour(1);

        assert_eq!(time.to_hours(), 1.0);
    }

    #[test]
    fn test_set_minutes() {
        let mut time = Time::new();
        time.set_minutes(1);

        assert_eq!(time.minutes, 1);
    }

    #[test]
    fn test_to_minutes() {
        let mut time = Time::new();
        time.set_minutes(1);

        assert_eq!(time.to_minutes(), 1.0);
    }

    #[test]
    fn test_set_seconds() {
        let mut time = Time::new();
        time.set_seconds(1);

        assert_eq!(time.seconds, 1);
    }

    #[test]
    fn test_to_seconds() {
        let mut time = Time::new();
        time.set_seconds(1);

        assert_eq!(time.to_seconds(), 1.0);
    }

    #[test]
    fn test_from_hours() {
        assert_eq!(
            12.508333.from_hours(),
            Time {
                hour: 12,
                minutes: 30,
                seconds: 30
            }
        );
    }

    #[test]
    fn test_from_minutes() {
        assert_eq!(
            61.50.from_minutes(),
            Time {
                hour: 1,
                minutes: 1,
                seconds: 30
            }
        );
    }

    #[test]
    fn test_from_seconds() {
        assert_eq!(
            3662.0.from_seconds(),
            Time {
                hour: 1,
                minutes: 1,
                seconds: 2
            }
        );
    }
}

fn parse(args: Args) {
    let re = Regex::new(r"^(?<hour>\d{1,5}):(?<minutes>\d{1,2}):(?<seconds>\d{1,2})$").unwrap();
    let Some(caps) = re.captures(&args.time) else {
        println!("Time argument must match 'hh:mm:ss' pattern");
        return;
    };

    let mut time = Time::new();

    time.set_hour(caps["hour"].parse().unwrap());
    time.set_minutes(caps["minutes"].parse().unwrap());
    time.set_seconds(caps["seconds"].parse().unwrap());

    #[cfg(debug_assertions)]
    println!("{:?}", time);

    match args.unit {
        's' => println!("{}", time.to_seconds()),
        'm' => println!("{}", time.to_minutes()),
        _ => println!("{}", time.to_hours()),
    }
}

fn decompose(args: Args) {
    let re = Regex::new(r"^\d+(.\d+$)?").unwrap();
    if re.is_match(&args.time) {
        let time = args.time.parse::<f32>().unwrap();

        #[cfg(debug_assertions)]
        println!("{:?}", time);

        match args.unit {
            's' => println!("{}", time.from_seconds()),
            'm' => println!("{}", time.from_minutes()),
            _ => println!("{}", time.from_hours()),
        }
    } else {
        println!("Time argument must match '\\d+(./\\d+)' pattern when decompose option is used");
    }
}

fn main() {
    let args = Args::parse();

    match args.decompose {
        true => decompose(args),
        false => parse(args),
    }
}
