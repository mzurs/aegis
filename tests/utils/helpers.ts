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
  return BigInt(numberValue * baseOfEight);
};

// Convert human readable number to e18s format in BigInt
export const humanToE18s = (numberValue: number) => {
  return BigInt(numberValue * baseOfEighteen);
};
