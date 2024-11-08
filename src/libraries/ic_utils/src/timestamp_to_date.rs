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
