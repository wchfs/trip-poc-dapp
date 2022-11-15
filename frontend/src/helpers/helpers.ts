import { BigNumber, ethers } from "ethers";
import { DateTime } from 'luxon';

export const hex2str = (hex: string) => {
  try {
    return ethers.utils.toUtf8String(hex);
  } catch (e) {
    // cannot decode hex payload as a UTF-8 string
    return hex;
  }
};

export const eth2gwei = (eth: string | undefined): BigNumber => {
  if (!eth) {
    throw new Error('eth2gwei: eth is undefined');
  }

  return ethers.utils.parseUnits(eth, "gwei");
}

export const gwei2eth = (gwei: string | undefined): string => {
  if (!gwei) {
    throw new Error('gwei2eth: gwei is undefined');
  }

  return ethers.utils.formatUnits(gwei, "gwei");
}
