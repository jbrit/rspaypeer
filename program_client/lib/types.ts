// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import type {Schema} from 'borsh';
import type {Decoded} from "./utils";
import {PublicKey} from "@solana/web3.js";
import { deserialize } from "borsh";

export interface SwapAuthority {
  random: number;
}

export const decodeSwapAuthority = (decoded: Decoded): SwapAuthority => ({
    random: decoded["random"] as number,
});

export const SwapAuthoritySchema: Schema =  {
    struct: {
        random: "u8",
    }
};

/// store the relevant information to track the info for the pool.
export interface Pool {
  ngn: PublicKey;
  usd: PublicKey;
}

export const decodePool = (decoded: Decoded): Pool => ({
    ngn: new PublicKey(decoded["ngn"] as Uint8Array),
    usd: new PublicKey(decoded["usd"] as Uint8Array),
});

export const PoolSchema: Schema =  {
    struct: {
        ngn: { array: { type: "u8", len: 32 } },
        usd: { array: { type: "u8", len: 32 } },
    }
};



