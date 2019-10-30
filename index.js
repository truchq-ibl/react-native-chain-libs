import { NativeModules } from 'react-native';
import { decode as base64_decode } from 'base-64';

const { ChainLibs } = NativeModules;

function Uint8ArrayFromB64(base64_string) {
    return Uint8Array.from(base64_decode(base64_string), c => c.charCodeAt(0));
}

class Ptr {
    static _wrap(ptr, klass) {
        const obj = Object.create(klass.prototype);
        obj.ptr = ptr;
        return obj;
    }

    static _assertClass(ptr, klass) {
        if (!(ptr instanceof klass)) {
            throw new Error(`expected instance of ${klass.name}`);
        }
        return ptr.ptr;
    }

    constructor() {
        throw new Error("Can't be initialized with constructor");
    }

    /**
    * Frees the pointer
    * @returns {void}
    */
    free() {
        if (!this.ptr) {
            throw new Error("Already freed!");
        }
        const ptr = this.ptr;
        this.ptr = null;
        ChainLibs.ptrFree(ptr);
    }
}

/**
* Allow to differentiate between address in
* production and testing setting, so that
* one type of address is not used in another setting.
* Example
* ```javascript
* let discriminant = AddressDiscrimination.Test;
* let address = Address::single_from_public_key(public_key, discriminant);
* ```
*/
export const AddressDiscrimination = ChainLibs.AddressDiscrimination;

export class Value extends Ptr {
    /**
    * Parse the given string into a rust u64 numeric type.
    * @param {string} s
    * @returns {Promise<Value>}
    */
    static async from_str(s) {
        const ret = await ChainLibs.valueFromStr(s);
        return Ptr._wrap(ret, Value);
    }
    /**
    * Return the wrapped u64 formatted as a string.
    * @returns {Promise<string>}
    */
    to_str() {
        return ChainLibs.valueToStr(this.ptr);
    }
    /**
    * @param {Value} other
    * @returns {Promise<Value>}
    */
    async checked_add(other) {
        const ret = await ChainLibs.valueCheckedAdd(this.ptr, Ptr._assertClass(other, Value));
        return Ptr._wrap(ret, Value);
    }
    /**
    * @param {Value} other
    * @returns {Promise<Value>}
    */
    async checked_sub(other) {
        const ret = await ChainLibs.valueCheckedSub(this.ptr, Ptr._assertClass(other, Value));
        return Ptr._wrap(ret, Value);
    }
}

/**
* ED25519 key used as public key
*/
export class PublicKey extends Ptr {
    /**
    * Get private key from its bech32 representation
    * Example:
    * ```javascript
    * const pkey = PublicKey.from_bech32("ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2");
    * ```
    * @param {string} bech32_str
    * @returns {Promise<PublicKey>}
    */
    static async from_bech32(bech32_str) {
        const ret = await ChainLibs.publicKeyFromBech32(bech32_str);
        return Ptr._wrap(ret, PublicKey);
    }
    /**
    * @returns {Promise<Uint8Array>}
    */
    async as_bytes() {
        const b64 = await ChainLibs.publicKeyAsBytes(this.ptr);
        return Uint8ArrayFromB64(b64);
    }
}

/**
* An address of any type, this can be one of
* * A utxo-based address without delegation (single)
* * A utxo-based address with delegation (group)
* * An address for an account
*/
export class Address extends Ptr {
    /**
    * Construct Address from its bech32 representation
    * Example
    * ```javascript
    * const address = Address.from_string("ca1q09u0nxmnfg7af8ycuygx57p5xgzmnmgtaeer9xun7hly6mlgt3pjyknplu");
    * ```
    * @param {string} s
    * @returns {Promise<Address>}
    */
    static async from_string(s) {
        const ret = await ChainLibs.addressFromString(s);
        return Ptr._wrap(ret, Address);
    }
    /**
    * Get Address bech32 (string) representation with a given prefix
    * ```javascript
    * let public_key = PublicKey.from_bech32(
    *     "ed25519_pk1kj8yvfrh5tg7n62kdcw3kw6zvtcafgckz4z9s6vc608pzt7exzys4s9gs8"
    * );
    * let discriminant = AddressDiscrimination.Test;
    * let address = Address.single_from_public_key(public_key, discriminant);
    * address.to_string("ta")
    * // ta1sj6gu33yw73dr60f2ehp6xemgf30r49rzc25gkrfnrfuuyf0mycgnj78ende550w5njvwzyr20q6rypdea597uu3jnwfltljddl59cseaq7yn9
    * ```
    * @param {string} prefix
    * @returns {Promise<string>}
    */
    to_string(prefix) {
        return ChainLibs.addressToString(this.ptr, prefix);
    }
    /**
    * Construct a single non-account address from a public key
    * ```javascript
    * let public_key = PublicKey.from_bech32(
    *     "ed25519_pk1kj8yvfrh5tg7n62kdcw3kw6zvtcafgckz4z9s6vc608pzt7exzys4s9gs8"
    * );
    * let address = Address.single_from_public_key(public_key, AddressDiscrimination.Test);
    * ```
    * @param {PublicKey} key
    * @param {number} discrimination
    * @returns {Promise<Address>}
    */
    static async single_from_public_key(key, discrimination) {
        const keyPtr = Ptr._assertClass(key, PublicKey);
        key.ptr = null;
        const ret = await ChainLibs.addressSingleFromPublicKey(keyPtr, discrimination);
        return Ptr._wrap(ret, Address);
    }
    /**
    * Construct a non-account address from a pair of public keys, delegating founds from the first to the second
    * @param {PublicKey} key
    * @param {PublicKey} delegation
    * @param {number} discrimination
    * @returns {Promise<Address>}
    */
    static async delegation_from_public_key(key, delegation, discrimination) {
        const keyPtr = Ptr._assertClass(key, PublicKey);
        const delPtr = Ptr._assertClass(delegation, PublicKey);
        key.ptr = delegation.ptr = null;
        const ret = await ChainLibs.addressDelegationFromPublicKey(keyPtr, delPtr, discrimination);
        return Ptr._wrap(ret, Address);
    }
    /**
    * Construct address of account type from a public key
    * @param {PublicKey} key
    * @param {number} discrimination
    * @returns {Promise<Address>}
    */
    static async account_from_public_key(key, discrimination) {
        const keyPtr = Ptr._assertClass(key, PublicKey);
        key.ptr = null;
        const ret = await ChainLibs.addressAccountFromPublicKey(keyPtr, discrimination);
        return Ptr._wrap(ret, Address);
    }
}

export class AuthenticatedTransaction extends Ptr {
    /**
    * Get a copy of the inner Transaction, discarding the signatures
    * @returns {Promise<Transaction>}
    */
    async transaction() {
        const ret = await ChainLibs.authenticatedTransactionTransaction(this.ptr);
        return Ptr._wrap(ret, Ptr);
    }
}

export class Fragment extends Ptr {
    /**
    * @param {AuthenticatedTransaction} tx
    * @returns {Promise<Fragment>}
    */
    static async from_authenticated_transaction(tx) {
        const txPtr = Ptr._assertClass(tx, AuthenticatedTransaction);
        tx.ptr = null;
        const ret = await ChainLibs.authenticatedTransactionTransaction(txPtr);
        return Ptr._wrap(ret, Fragment);
    }
    /**
    * Deprecated: Use `from_authenticated_transaction` instead
    * @param {AuthenticatedTransaction} tx
    * @returns {Promise<Fragment>}
    */
    static async from_generated_transaction(tx) {
        const txPtr = Ptr._assertClass(tx, AuthenticatedTransaction);
        tx.ptr = null;
        const ret = await ChainLibs.fragmentFromAuthenticatedTransaction(txPtr);
        return Ptr._wrap(ret, Fragment);
    }
    /**
    * Get a Transaction if the Fragment represents one
    * @returns {Promise<AuthenticatedTransaction>}
    */
    async get_transaction() {
        const ptr = this.ptr;
        this.ptr = null;
        const ret = await ChainLibs.fragmentGetTransaction(ptr);
        return Ptr._wrap(ret, AuthenticatedTransaction);
    }
    /**
    * @returns {Promise<Uint8Array>}
    */
    async as_bytes() {
        const b64 = await ChainLibs.fragmentAsBytes(this.ptr);
        return Uint8ArrayFromB64(b64);
    }
    /**
    * @returns {Promise<boolean>}
    */
    is_initial() {
        return ChainLibs.fragmentIsInitial(this.ptr);
    }
    /**
    * @returns {Promise<boolean>}
    */
    is_transaction() {
        return ChainLibs.fragmentIsTransaction(this.ptr);
    }
    /**
    * @returns {Promise<boolean>}
    */
    is_owner_stake_delegation() {
        return ChainLibs.fragmentIsOwnerStakeDelegation(this.ptr);
    }
    /**
    * @returns {Promise<boolean>}
    */
    is_stake_delegation() {
        return ChainLibs.fragmentIsStakeDelegation(this.ptr);
    }
    /**
    * @returns {Promise<boolean>}
    */
    is_pool_registration() {
        return ChainLibs.fragmentIsPoolRegistration(this.ptr);
    }
    /**
    * @returns {Promise<boolean>}
    */
    is_pool_management() {
        return ChainLibs.fragmentIsPoolManagement(this.ptr);
    }
    /**
    * @returns {Promise<boolean>}
    */
    is_old_utxo_declaration() {
        return ChainLibs.fragmentIsOldUtxoDeclaration(this.ptr);
    }
    /**
    * @returns {Promise<boolean>}
    */
    is_update_proposal() {
        return ChainLibs.fragmentIsUpdateProposal(this.ptr);
    }
    /**
    * @returns {Promise<boolean>}
    */
    is_update_vote() {
        return ChainLibs.fragmentIsUpdateVote(this.ptr);
    }
    /**
    * @returns {Promise<FragmentId>}
    */
    async id() {
        const ret = await ChainLibs.fragmentId(this.ptr);
        return Ptr._wrap(ret, Ptr);
    }
}

/**
* This is either an single account or a multisig account depending on the witness type
*/
export class Account extends Ptr {
    /**
    * @param {Address} address
    * @returns {Promise<Account>}
    */
    static async from_address(address) {
        const ret = await ChainLibs.accountFromAddress(Ptr._assertClass(address, Address));
        return Ptr._wrap(ret, Account);
    }
}

/**
*/
export class Input extends Ptr {
    /**
    * @param {Account} account
    * @param {Value} v
    * @returns {Promise<Input>}
    */
    static async from_account(account, v) {
        const accountPtr = Ptr._assertClass(account, Account);
        const vPtr = Ptr._assertClass(v, Value);
        v.ptr = null;
        const ret = await ChainLibs.inputFromAccount(accountPtr, vPtr);
        return Ptr._wrap(ret, Input);
    }
}

/**
* Algorithm used to compute transaction fees
* Currently the only implementation if the Linear one
*/
export class Fee extends Ptr {
    /**
    * Linear algorithm, this is formed by: `coefficient * (#inputs + #outputs) + constant + certificate * #certificate
    * @param {Value} constant
    * @param {Value} coefficient
    * @param {Value} certificate
    * @returns {Promise<Fee>}
    */
    static async linear_fee(constant, coefficient, certificate) {
        const constantPtr = Ptr._assertClass(constant, Value);
        const coefficientPtr = Ptr._assertClass(coefficient, Value);
        const certificatePtr = Ptr._assertClass(certificate, Value);
        constant.ptr = coefficient.ptr = certificate.ptr = null;
        const ret = await ChainLibs.feeLinearFee(constantPtr, coefficientPtr, certificatePtr);
        return Ptr._wrap(ret, Fee);
    }
}

/**
* Helper to add change addresses when finalizing a transaction, there are currently two options
* * forget: use all the excess money as fee
* * one: send all the excess money to the given address
*/
export class OutputPolicy extends Ptr {
    /**
    * don\'t do anything with the excess money in transaction
    * @returns {Promise<OutputPolicy>}
    */
    static async forget() {
        const ret = await ChainLibs.outputPolicyForget();
        return Ptr._wrap(ret, OutputPolicy);
    }
    /**
    * use the given address as the only change address
    * @param {Address} address
    * @returns {Promise<OutputPolicy>}
    */
    static async one(address) {
        const addressPtr = Ptr._assertClass(address, Address);
        address.ptr = null;
        const ret = await ChainLibs.outputPolicyOne(addressPtr);
        return Ptr._wrap(ret, OutputPolicy);
    }
}

/**
* Builder pattern implementation for making a Transaction
*
* Example
*
* ```javascript
* const txbuilder = new TransactionBuilder();
*
* const account = Account.from_address(Address.from_string(
*   &#39;ca1qh9u0nxmnfg7af8ycuygx57p5xgzmnmgtaeer9xun7hly6mlgt3pj2xk344&#39;
* ));
*
* const input = Input.from_account(account, Value.from_str(\'1000\'));
*
* txbuilder.add_input(input);
*
* txbuilder.add_output(
*   Address.from_string(
*     &#39;ca1q5nr5pvt9e5p009strshxndrsx5etcentslp2rwj6csm8sfk24a2w3swacn&#39;
*   ),
*   Value.from_str(\'500\')
* );
*
* const feeAlgorithm = Fee.linear_fee(
*   Value.from_str(\'20\'),
*   Value.from_str(\'5\'),
*   Value.from_str(\'0\')
* );
*
* const finalizedTx = txbuilder.finalize(
*   feeAlgorithm,
*   OutputPolicy.one(accountInputAddress)
* );
* ```
*/
export class TransactionBuilder extends Ptr {
    /**
    * Create a TransactionBuilder for a transaction without certificate
    * @returns {Promise<TransactionBuilder>}
    */
    static async new_no_payload() {
        const ret = await ChainLibs.transactionBuilderNewNoPayload();
        return Ptr._wrap(ret, TransactionBuilder);
    }

    /**
    * Add input to the transaction
    * @param {Input} input
    * @returns {Promise<void>}
    */
    add_input(input) {
        const inputPtr = Ptr._assertClass(input, Input);;
        input.ptr = null;
        return ChainLibs.transactionBuilderAddInput(this.ptr, inputPtr);
    }
    /**
    * Add output to the transaction
    * @param {Address} address
    * @param {Value} value
    * @returns {Promise<void>}
    */
    add_output(address, value) {
        const addressPtr = Ptr._assertClass(address, Address);
        const valuePtr = Ptr._assertClass(value, Value);
        address.ptr = value.ptr = null;
        return ChainLibs.transactionBuilderAddOutput(this.ptr, addressPtr, valuePtr);
    }

    /**
    * Finalize the transaction by adding the change Address output
    * leaving enough for paying the minimum fee computed by the given algorithm
    * see the unchecked_finalize for the non-assisted version
    *
    * Example
    *
    * ```javascript
    * const feeAlgorithm = Fee.linear_fee(
    *     Value.from_str(\'20\'), Value.from_str(\'5\'), Value.from_str(\'10\')
    * );
    *
    * const finalizedTx = txbuilder.finalize(
    *   feeAlgorithm,
    *   OutputPolicy.one(changeAddress)
    * );
    * ```
    * @param {Fee} fee
    * @param {OutputPolicy} outputPolicy
    * @returns {Promise<Transaction>}
    */
    async seal_with_output_policy(fee, outputPolicy) {
        const feePtr = Ptr._assertClass(fee, Fee);
        const outputPolicyPtr = Ptr._assertClass(outputPolicy, OutputPolicy);
        const ptr = this.ptr;
        this.ptr = outputPolicy.ptr = null;
        const ret = await ChainLibs.transactionBuilderSealWithOutputPolicy(ptr, feePtr, outputPolicyPtr);
        return Ptr._wrap(ret, Ptr);
    }
}

/**
* Type for representing a generic Hash
*/
export class Hash extends Ptr {
    /**
    * @param {string} hex_string
    * @returns {Promise<Hash>}
    */
    static async from_hex(hexString) {
        const ret = await ChainLibs.hashFromHex(hexString);
        return Ptr._wrap(ret, Hash);
    }
}

/**
* ED25519 signing key, either normal or extended
*/
export class PrivateKey extends Ptr {
    /**
    * Get private key from its bech32 representation
    * ```javascript
    * PrivateKey.from_bech32(&#39;ed25519_sk1ahfetf02qwwg4dkq7mgp4a25lx5vh9920cr5wnxmpzz9906qvm8qwvlts0&#39;);
    * ```
    * For an extended 25519 key
    * ```javascript
    * PrivateKey.from_bech32(&#39;ed25519e_sk1gqwl4szuwwh6d0yk3nsqcc6xxc3fpvjlevgwvt60df59v8zd8f8prazt8ln3lmz096ux3xvhhvm3ca9wj2yctdh3pnw0szrma07rt5gl748fp&#39;);
    * ```
    * @param {string} bech32_str
    * @returns {Promise<PrivateKey>}
    */
    static async from_bech32(bech32Str) {
        const ret = await ChainLibs.privateKeyFromBech32(bech32Str);
        return Ptr._wrap(ret, PrivateKey);
    }
}

/**
*/
export class SpendingCounter extends Ptr {
    /**
    * @returns {Promise<SpendingCounter>}
    */
    static async zero() {
        const ret = await ChainLibs.spendingCounterZero();
        return Ptr._wrap(ret, SpendingCounter);
    }
}

/**
* Structure that proofs that certain user agrees with
* some data. This structure is used to sign `Transaction`
* and get `SignedTransaction` out.
*
* It\'s important that witness works with opaque structures
* and may not know the contents of the internal transaction.
*/
export class Witness extends Ptr {
    /**
    * Generate Witness for an account based transaction Input
    * the account-spending-counter should be incremented on each transaction from this account
    * @param {Hash} genesisHash
    * @param {TransactionSignDataHash} transactionId
    * @param {PrivateKey} secretKey
    * @param {SpendingCounter} accountSpendingCounter
    * @returns {Promise<Witness>}
    */
    static async for_account(genesisHash, transactionId, secretKey, accountSpendingCounter) {
        const genesisHashPtr = Ptr._assertClass(genesisHash, Hash);
        const transactionIdPtr = Ptr._assertClass(transactionId, Ptr);
        const secretKeyPtr = Ptr._assertClass(secretKey, PrivateKey);
        const accountSpendingCounterPtr = Ptr._assertClass(accountSpendingCounter, SpendingCounter);
        genesisHash.ptr = transactionId.ptr = secretKey.ptr = accountSpendingCounter.ptr = null;
        const ret = await ChainLibs.witnessForAccount(genesisHashPtr, transactionIdPtr, secretKeyPtr, accountSpendingCounterPtr);
        return Ptr._wrap(ret, Witness);
    }
}

/**
* Builder pattern implementation for signing a Transaction (adding witnesses)
* Example (for an account as input)
*
* ```javascript
* //finalizedTx could be the result of the finalize method on a TransactionBuilder object
* const finalizer = new TransactionFinalizer(finalizedTx);
*
* const witness = Witness.for_account(
*   Hash.from_hex(genesisHashString),
*   finalizer.get_txid(),
*   inputAccountPrivateKey,
*   SpendingCounter.zero()
* );
*
* finalizer.set_witness(0, witness);
*
* const signedTx = finalizer.build();
* ```
*/
export class TransactionFinalizer extends Ptr {
    /**
    * @param {Transaction} transaction
    * @returns {Promise<TransactionFinalizer>}
    */
    static async new(transaction) {
        const transactionPtr = Ptr._assertClass(transaction, Ptr);
        transaction.ptr = null;
        const ret = await ChainLibs.transactionFinalizerNew(transactionPtr);
        return Ptr._wrap(ret, TransactionFinalizer);
    }
    /**
    * Set the witness for the corresponding index, the index corresponds to the order in which the inputs were added to the transaction
    * @param {number} index
    * @param {Witness} witness
    * @returns {Promise<void>}
    */
    set_witness(index, witness) {
        const witnessPtr = Ptr._assertClass(witness, Witness);
        witness.ptr = null;
        return ChainLibs.transactionFinalizerSetWitness(this.ptr, index, witnessPtr);
    }
    /**
    * @returns {Promise<TransactionSignDataHash>}
    */
    async get_tx_sign_data_hash() {
        const ret = await ChainLibs.transactionFinalizerGetTxSignDataHash(this.ptr);
        return Ptr._wrap(ret, Ptr);
    }
    /**
    * Deprecated: Use `get_tx_sign_data_hash` instead\
    * @returns {Promise<AuthenticatedTransaction>}
    */
    async build() {
        const ptr = this.ptr;
        this.ptr = null;
        const ret = await ChainLibs.transactionFinalizerBuild(ptr);
        return Ptr._wrap(ret, AuthenticatedTransaction);
    }
}
