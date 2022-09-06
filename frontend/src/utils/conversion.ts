export const arrayIntToHexStr = (input: Array<number>): string => {
  const hex = input
    .map((x) => {
      let v = (x + 0xffffffff + 1).toString(16);
      v = "00000000" + v;
      v = v.substring(v.length - 8);
      return v;
    })
    .join("");
  return hex;
};

export const hexStrToArrayInt = (input: string): Array<number> => {
  const k: Array<number> = [];
  while (input.length) {
    let x = parseInt(input.substring(0, 8), 16);
    x = (x + 0xffffffff + 1) & 0xffffffff;
    k.push(x);
    input = input.substring(8);
  }
  return k;
};
