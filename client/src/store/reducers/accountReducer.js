import { Keypair } from "@solana/web3.js";
import { accountTypes } from "../actionTypes";

const programSecretKeyString =
  "[199,88,92,228,28,186,219,81,18,82,22,7,5,10,135,202,224,116,53,172,72,17,15,67,2,151,1,103,44,15,76,81,110,185,79,56,108,176,110,188,187,215,220,106,179,109,187,189,23,37,93,235,52,39,232,77,177,115,58,87,120,94,187,46]";
const programSecretKey = Uint8Array.from(JSON.parse(programSecretKeyString));
const programKeypair = Keypair.fromSecretKey(programSecretKey);

const initialState = {
  loading: false,
  isError: false,
  errMsg: null,
  wallet: null,
  accountId: "",
  programId: programKeypair.publicKey,
};

export function account(state = initialState, action) {
  switch (action.type) {
    case accountTypes.CREATE_REQUEST:
      return {
        ...state,
        isError: false,
        errMsg: null,
        loading: true,
      };
    case accountTypes.CREATE_SUCCESS:
      return {
        ...state,
        loading: false,
        wallet: action.payload.wallet,
        accountId: action.payload.derivedAddress,
      };
    case accountTypes.CREATE_FAILURE:
      return {
        ...state,
        loading: false,
        isError: true,
        errMsg: action.payload.error,
      };
    default:
      return state;
  }
}
