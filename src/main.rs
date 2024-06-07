use time_util::seconds_since;
mod time_util;

fn main() {
    let date = "2024-06-07T00:00:00.000Z";
    let duration = seconds_since(date);
    println!("Date: {:?}", duration);
}
