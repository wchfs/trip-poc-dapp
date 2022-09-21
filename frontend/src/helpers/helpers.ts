import { ethers } from "ethers";

export const hex2str = (hex: string) => {
  try {
    return ethers.utils.toUtf8String(hex);
  } catch (e) {
    // cannot decode hex payload as a UTF-8 string
    return hex;
  }
};
