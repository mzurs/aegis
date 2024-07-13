export function currentTimePlusExtraMinutesInNanoseconds(extraTime:number): bigint {
    const now = Date.now(); // Get current time in milliseconds
    const fiveMinutesInMilliseconds = extraTime * 60 * 1000; // Convert 5 minutes to milliseconds
    const fiveMinutesInNanoseconds = BigInt(now + fiveMinutesInMilliseconds) * 1_000_000n; // Convert to nanoseconds with bigint
    return fiveMinutesInNanoseconds;
  }