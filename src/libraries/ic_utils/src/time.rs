use time::OffsetDateTime;

///
/// Convert the timestamp into date + time
///
pub fn convert_to_datetime(timestamp_ns: u64) -> String {
    //ic_cdk::api::time() returns ns of timestamp
    let timestamp_s = timestamp_ns / 1_000_000_000;
    // core funtion here : from_unix_timestamp()
    let date: OffsetDateTime = OffsetDateTime::from_unix_timestamp(timestamp_s as i64).unwrap();
    format!(
        "{:04}_{:02}_{:02}_{:02}_{:02}_{:02}",
        date.year(),
        date.month() as u32,
        date.day(),
        date.hour(),
        date.minute(),
        date.second()
    )
}

///
/// Convert the timestamp into the date
///
pub fn convert_to_date(timestamp_ns: u64) -> String {
    //ic_cdk::api::time() returns ns of timestamp
    let timestamp_s = timestamp_ns / 1_000_000_000;
    // core funtion here : from_unix_timestamp()
    let date = OffsetDateTime::from_unix_timestamp(timestamp_s as i64).unwrap();

    format!("{:04}{:02}{:02}", date.year(), date.month() as u32, date.day(),)
}

///
/// Calculate the remaining time in years given a past timestamp in nanoseconds
///
pub fn remaining_time_in_years(past_timestamp_ns: u64, current_timestamp_ns: u64) -> f32 {
    if current_timestamp_ns - past_timestamp_ns <= 0 {
        return 0.00;
    }
    const NANOS_IN_A_YEAR: f64 = 365.25 * 24.0 * 60.0 * 60.0 * 100_000_000_000.0;
    //31_536_000_000_000_000; // Approximate number of nanoseconds in a year

    ic_cdk::println!("{}", NANOS_IN_A_YEAR);

    // Calculate the difference in nanoseconds
    let duration_ns: u64 = current_timestamp_ns.saturating_sub(past_timestamp_ns);
    ic_cdk::println!("{}", current_timestamp_ns);
    ic_cdk::println!("{}", past_timestamp_ns);

    ic_cdk::println!("{}", duration_ns);

    // Convert nanoseconds to years
    (duration_ns as f64 / NANOS_IN_A_YEAR) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_timestamp_to_date() {
        let date: String = convert_to_date(1686184163560519943);

        println!("Date: {}", date.to_owned());

        let year: String = date[0..=3].to_owned();
        assert_eq!(year, "2023".to_owned());
    }

    #[test]
    fn test_remaining_time_in_years() {
        let past_timestamp_ns = 1731316448000 * 1000000; // Example timestamp in nanoseconds
        let years = remaining_time_in_years(past_timestamp_ns, 1733908448000 * 1000000);
        println!("Remaining time in years: {}", years);
        assert_eq!((years * 100.0).round() / 100.0 as f32, 0.08 as f32);
        // Depending on your test case, you can use an assertion here
    }
}
