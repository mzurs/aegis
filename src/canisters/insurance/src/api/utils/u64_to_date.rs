pub fn convert_timestamp_to_date(timestamp_ns: u64) -> String {
    // Constants for conversion
    const NANOS_PER_SECOND: u64 = 1_000_000_000;
    const DAYS_SINCE_EPOCH_BASE: i64 = 719163; // Adjust for your desired epoch (1970-01-01)
    const HOURS_PER_DAY: u32 = 24;

    // Convert nanoseconds to seconds and remaining nanoseconds
    let seconds = timestamp_ns / NANOS_PER_SECOND;
    // let nanos = (timestamp_ns % NANOS_PER_SECOND) as u32;

    // Calculate days since epoch (approximate)
    let mut days_since_epoch = (seconds / 86400) as i64; // 86400 seconds per day

    // Adjust for leap years (rough approximation, might be off by a day)
    let leap_years = seconds as i64 / (365 * 6 * HOURS_PER_DAY) as i64; // Approximate leap years
    days_since_epoch -= leap_years;

    // Add base days for your epoch
    days_since_epoch += DAYS_SINCE_EPOCH_BASE;

    // Extract year, month, and day
    let mut year = days_since_epoch / 365;
    let mut remaining_days = days_since_epoch % 365;

    // Approximate month (assuming 30 days per month, adjust for accuracy)
    let mut month = remaining_days / 30;
    remaining_days %= 30;

    // Adjust year for leap years (rough approximation)
    if month == 2 && remaining_days == 29 && (year % 4 == 0 && year % 100 != 0 || year % 400 == 0) {
        month = 1; // Leap year adjustment for February 29th
    } else if month > 2 && (year % 4 == 0 && year % 100 != 0 || year % 400 == 0) {
        year -= 1; // Leap year adjustment for other months
    }

    // Adjust month and day based on month lengths (approximate)
    let month_lengths = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    while remaining_days >= month_lengths[month as usize] {
        remaining_days -= month_lengths[month as usize];
        month += 1;
    }

    // Format the date string
    format!("{:04}-{:02}-{:02}", year + 1, month + 1, remaining_days + 1) // Add 1 for human-readable format
}
