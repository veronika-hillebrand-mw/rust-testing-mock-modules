use time_util_mock_struct::{
    seconds_since as seconds_since_struct, ChronoClock as ChronoClockStruct,
};
mod time_util_mock_enum;
mod time_util_mock_struct;
mod time_util_mockall;
use time_util_mock_enum::{seconds_since as seconds_since_enum, ChronoClock as ChronoClockEnum};
use time_util_mockall::{
    seconds_since as seconds_since_mockall, ChronoClock as ChronoClockMockall,
};

fn main() {
    let date = "2024-06-07T00:00:00.000Z";
    let duration_struct = seconds_since_struct(date, ChronoClockStruct);
    println!("Duration, struct: {:?}", duration_struct);
    let duration_enum = seconds_since_enum::<ChronoClockEnum>(date);
    println!("Duration, enum: {:?}", duration_enum);
    let duration_mockall = seconds_since_mockall(date, &ChronoClockMockall);
    println!("Duration, mockall: {:?}", duration_mockall);
}
