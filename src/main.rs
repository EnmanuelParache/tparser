use clap::Parser;
use regex::Regex;

/// Time units supported
#[derive(Debug, Clone)]
enum TimeUnits {
    Hour,
    Minutes,
    Seconds,
}

/// Workaround to convert u8 to TimeUnits
/// and avoid implementing many other traits
/// required for Args Parser
impl From<u8> for TimeUnits {
    fn from(value: u8) -> Self {
        match value {
            0 => TimeUnits::Hour,
            1 => TimeUnits::Minutes,
            2 => TimeUnits::Seconds,
            _ => TimeUnits::Hour,
        }
    }
}

/// Simple program to parse time
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Time to be parsed
    #[arg(short, long)]
    time: String,

    /// 0: Hours
    /// 1: Minutes
    /// 2: Seconds
    #[arg(short, long, default_value_t = 0)]
    unit: u8,
}

/// Trait to convert to different
/// time units
trait ToTimeUnits {
    fn to_hours(&self) -> f32;
    fn to_minutes(&self) -> f32;
    fn to_seconds(&self) -> f32;
}

/// Time struct to hold
/// hour, minutes and seconds
#[derive(Debug)]
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
}

fn main() {
    let args = Args::parse();
    let re = Regex::new(r"^(?<hour>\d{1,5}):(?<minutes>\d{1,2}):(?<seconds>\d{1,2})$").unwrap();

    let Some(caps) = re.captures(&args.time) else {
        println!("No match");
        return;
    };

    let mut time = Time::new();

    time.set_hour(caps["hour"].parse().unwrap());
    time.set_minutes(caps["minutes"].parse().unwrap());
    time.set_seconds(caps["seconds"].parse().unwrap());

    #[cfg(debug_assertions)]
    println!("{:?}", time);

    match args.unit.into() {
        TimeUnits::Hour => println!("{}", time.to_hours()),
        TimeUnits::Minutes => println!("{}", time.to_minutes()),
        TimeUnits::Seconds => println!("{}", time.to_seconds()),
    }
}
