// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import * as pda from "./pda";
import * as T from "./types";
import {
    Commitment,
    Connection,
    GetAccountInfoConfig,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    TransactionSignature,
} from "@solana/web3.js";
import {deserialize, serialize} from "borsh";


let _programId: PublicKey;
let _connection: Connection;

export const initializeClient = (
    programId: PublicKey,
    connection: Connection
) => {
    _programId = programId;
    _connection = connection;
};

export enum PaypeerInstruction {
/**
 * Initialize a Pool account for this contract.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` swap_authority: {@link SwapAuthority} 
 * 2. `[writable]` pool: {@link Pool} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - ngn_mint: {@link PublicKey} 
 * - usd_mint: {@link PublicKey} 
 */
    SetPool = 0,

/**
 * Initialize Swap
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` pool: {@link Pool} 
 * 2. `[]` token_program: {@link PublicKey} 
 * 3. `[]` user_authority: {@link PublicKey} 
 * 4. `[writable]` user_source: {@link PublicKey} 
 * 5. `[writable]` user_destination: {@link PublicKey} 
 * 6. `[writable]` swap_authority: {@link SwapAuthority} 
 * 7. `[writable]` swap_source: {@link PublicKey} 
 * 8. `[writable]` swap_destination: {@link PublicKey} 
 * 9. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - amount_in: {@link BigInt} 
 * - min_amount_out: {@link BigInt} 
 */
    Swap = 1,
}

export type SetPoolArgs = {
    feePayer: PublicKey;
    ngnMint: PublicKey;
    usdMint: PublicKey;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Initialize a Pool account for this contract.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` swap_authority: {@link SwapAuthority} 
 * 2. `[writable]` pool: {@link Pool} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - ngn_mint: {@link PublicKey} 
 * - usd_mint: {@link PublicKey} 
 */
export const setPool = (args: SetPoolArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                ngn_mint: { array: { type: "u8", len: 32 } },
                usd_mint: { array: { type: "u8", len: 32 } },
            },
        },
        {
            id: PaypeerInstruction.SetPool,
            ngn_mint: args.ngnMint.toBytes(),
            usd_mint: args.usdMint.toBytes(),
        }
    );

    const [swapAuthorityPubkey] = pda.deriveSwapAuthorityPDA(_programId);
    const [poolPubkey] = pda.derivePoolPDA(_programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: swapAuthorityPubkey, isSigner: false, isWritable: true},
            {pubkey: poolPubkey, isSigner: false, isWritable: true},
            {pubkey: new PublicKey("11111111111111111111111111111111"), isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Initialize a Pool account for this contract.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable]` swap_authority: {@link SwapAuthority} 
 * 2. `[writable]` pool: {@link Pool} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - ngn_mint: {@link PublicKey} 
 * - usd_mint: {@link PublicKey} 
 */
export const setPoolSendAndConfirm = async (
    args: Omit<SetPoolArgs, "feePayer"> & { 
        signers: { feePayer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(setPool({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, ]
    );
};

export type SwapArgs = {
    feePayer: PublicKey;
    tokenProgram: PublicKey;
    userAuthority: PublicKey;
    userSource: PublicKey;
    userDestination: PublicKey;
    swapSource: PublicKey;
    swapDestination: PublicKey;
    amountIn: bigint;
    minAmountOut: bigint;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Initialize Swap
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` pool: {@link Pool} 
 * 2. `[]` token_program: {@link PublicKey} 
 * 3. `[]` user_authority: {@link PublicKey} 
 * 4. `[writable]` user_source: {@link PublicKey} 
 * 5. `[writable]` user_destination: {@link PublicKey} 
 * 6. `[writable]` swap_authority: {@link SwapAuthority} 
 * 7. `[writable]` swap_source: {@link PublicKey} 
 * 8. `[writable]` swap_destination: {@link PublicKey} 
 * 9. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - amount_in: {@link BigInt} 
 * - min_amount_out: {@link BigInt} 
 */
export const swap = (args: SwapArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                amount_in: "u64",
                min_amount_out: "u64",
            },
        },
        {
            id: PaypeerInstruction.Swap,
            amount_in: args.amountIn,
            min_amount_out: args.minAmountOut,
        }
    );

    const [poolPubkey] = pda.derivePoolPDA(_programId);
    const [swapAuthorityPubkey] = pda.deriveSwapAuthorityPDA(_programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: poolPubkey, isSigner: false, isWritable: false},
            {pubkey: args.tokenProgram, isSigner: false, isWritable: false},
            {pubkey: args.userAuthority, isSigner: false, isWritable: false},
            {pubkey: args.userSource, isSigner: false, isWritable: true},
            {pubkey: args.userDestination, isSigner: false, isWritable: true},
            {pubkey: swapAuthorityPubkey, isSigner: false, isWritable: true},
            {pubkey: args.swapSource, isSigner: false, isWritable: true},
            {pubkey: args.swapDestination, isSigner: false, isWritable: true},
            {pubkey: new PublicKey("11111111111111111111111111111111"), isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Initialize Swap
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` pool: {@link Pool} 
 * 2. `[]` token_program: {@link PublicKey} 
 * 3. `[]` user_authority: {@link PublicKey} 
 * 4. `[writable]` user_source: {@link PublicKey} 
 * 5. `[writable]` user_destination: {@link PublicKey} 
 * 6. `[writable]` swap_authority: {@link SwapAuthority} 
 * 7. `[writable]` swap_source: {@link PublicKey} 
 * 8. `[writable]` swap_destination: {@link PublicKey} 
 * 9. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - amount_in: {@link BigInt} 
 * - min_amount_out: {@link BigInt} 
 */
export const swapSendAndConfirm = async (
    args: Omit<SwapArgs, "feePayer"> & { 
        signers: { feePayer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(swap({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, ]
    );
};

// Getters

export const getSwapAuthority = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.SwapAuthority | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeSwapAuthority(deserialize(T.SwapAuthoritySchema, buffer.data) as Record<string, unknown>);
}

export const getPool = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.Pool | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodePool(deserialize(T.PoolSchema, buffer.data) as Record<string, unknown>);
}


// Websocket Events

