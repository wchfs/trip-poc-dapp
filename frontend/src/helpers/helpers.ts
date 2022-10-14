import { ethers } from "ethers";

export const hex2str = (hex: string) => {
  try {
    return ethers.utils.toUtf8String(hex);
  } catch (e) {
    // cannot decode hex payload as a UTF-8 string
    return hex;
  }
};

export const eth2gwei = (eth: string) => {
  return ethers.utils.parseUnits(eth, "gwei");
}

export const gwei2eth = (gwei: string) => {
  return ethers.utils.formatUnits(gwei, "gwei");
}
