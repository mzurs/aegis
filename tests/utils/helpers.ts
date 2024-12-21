const baseOfEight = 100000000;
const baseOfEighteen = 1_000_000_000_000_000_000;

// Convert to BigInt e8s format to human readable
export const e8sToHuman = (bigIntValue: any) => {
  return Number(bigIntValue) / baseOfEight;
};
// Convert to BigInt e8s format to human readable
export const e18sToHuman = (bigIntValue: any) => {
  return Number(bigIntValue) / baseOfEighteen;
};
// Convert human readable number to e8s format in BigInt
export const humanToE8s = (numberValue: number) => {
  return BigInt(Math.trunc(numberValue * baseOfEight));
};

// Convert human readable number to e18s format in BigInt
export const humanToE18s = (numberValue: number) => {
  return BigInt(Math.trunc(numberValue * baseOfEighteen));
};

function convertNanoToMilli(nanoseconds: bigint): number {
  return Number(nanoseconds) / 1000000;
}

function getTimeFromNano(nanoseconds: bigint): Date {
  const milliseconds = convertNanoToMilli(nanoseconds);
  return new Date(milliseconds);
}

export function getFormattedTime(nanoseconds: bigint): string {
  const date = getTimeFromNano(nanoseconds);
  const year = date.getFullYear();
  const month = date.getMonth() + 1; // Months are zero-indexed (January is 0)
  const day = date.getDate();
  const hours = date.getHours();
  const minutes = date.getMinutes();
  const seconds = date.getSeconds();
  const milliseconds = date.getMilliseconds();

  // Format the date and time as needed
  return `${year}-${month.toString().padStart(2, "0")}-${day
    .toString()
    .padStart(2, "0")} ${hours.toString().padStart(2, "0")}:${minutes
    .toString()
    .padStart(2, "0")}:${seconds.toString().padStart(2, "0")}.${milliseconds
    .toString()
    .padStart(3, "0")}`;
}
