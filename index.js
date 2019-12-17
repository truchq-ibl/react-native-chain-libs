import { NativeModules } from 'react-native';
import { decode as base64_decode, encode as base64_encode } from 'base-64';

const { ChainLibs } = NativeModules;

function Uint8ArrayFromB64(base64_string) {
    return Uint8Array.from(base64_decode(base64_string), c => c.charCodeAt(0));
}

function b64FromUint8Array(uint8Array) {
    return base64_encode(String.fromCharCode.apply(null, uint8Array));
}

class Ptr {
    static _wrap(ptr, klass) {
        if (ptr === '0') {
            return undefined;
        }
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
    * @returns {Promise<void>}
    */
    async free() {
        if (!this.ptr) {
            return;
        }
        const ptr = this.ptr;
        this.ptr = null;
        await ChainLibs.ptrFree(ptr);
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

/**
*/
export const AddressKind = ChainLibs.AddressKind;

/**
*/
export const CertificateKind = ChainLibs.CertificateKind;

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
*/
export class SingleAddress extends Ptr {}

/**
*/
export class GroupAddress extends Ptr {}

/**
*/
export class AccountAddress extends Ptr {}

/**
* An address of any type, this can be one of
* * A utxo-based address without delegation (single)
* * A utxo-based address with delegation (group)
* * An address for an account
*/
export class Address extends Ptr {
    /**
    * @param {Uint8Array} bytes
    * @returns {Promise<Address>}
    */
    static async from_bytes(bytes) {
        const ret = await ChainLibs.addressFromBytes(b64FromUint8Array(bytes));
        return Ptr._wrap(ret, Address);
    }

    /**
    * @returns {Promise<Uint8Array>}
    */
    async as_bytes() {
        const b64 = await ChainLibs.addressAsBytes(this.ptr);
        return Uint8ArrayFromB64(b64);
    }

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
        const ret = await ChainLibs.addressAccountFromPublicKey(keyPtr, discrimination);
        return Ptr._wrap(ret, Address);
    }

    /**
    * @returns {Promise<number>}
    */
    get_discrimination() {
        return ChainLibs.addressGetDiscrimination(this.ptr);
    }

    /**
    * @returns {Promise<number>}
    */
    get_kind() {
        return ChainLibs.addressGetKind(this.ptr);
    }

    /**
    * @returns {Promise<SingleAddress | undefined>}
    */
    async to_single_address() {
        const ret = await ChainLibs.addressToSingleAddress(this.ptr);
        return Ptr._wrap(ret, SingleAddress);
    }

    /**
    * @returns {Promise<GroupAddress | undefined>}
    */
    async to_group_address() {
        const ret = await ChainLibs.addressToGroupAddress(this.ptr);
        return Ptr._wrap(ret, GroupAddress);
    }

    /**
    * @returns {Promise<AccountAddress | undefined>}
    */
    async to_account_address() {
        const ret = await ChainLibs.addressToAccountAddress(this.ptr);
        return Ptr._wrap(ret, AccountAddress);
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

    /**
    * @param {PublicKey} key
    * @returns {Promise<Account>}
    */
    static async single_from_public_key(key) {
        const keyPtr = Ptr._assertClass(key, PublicKey);
        const ret = await ChainLibs.accountSingleFromPublicKey(keyPtr);
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
        const ret = await ChainLibs.inputFromAccount(accountPtr, vPtr);
        return Ptr._wrap(ret, Input);
    }

    /**
    * @returns {Promise<Value>}
    */
    async value() {
        const ret = await ChainLibs.inputValue(this.ptr);
        return Ptr._wrap(ret, Value);
    }
}

/**
*/
export class Inputs extends Ptr {
    /**
    * @returns {Promise<Inputs>}
    */
    static async new() {
        const ret = await ChainLibs.inputsNew();
        return Ptr._wrap(ret, Inputs);
    }

    /**
    * @returns {Promise<number>}
    */
    size() {
        return ChainLibs.inputsSize(this.ptr);
    }

    /**
    * @param {number} index
    * @returns {Promise<Input>}
    */
    async get(index) {
        const ret = await ChainLibs.inputsGet(this.ptr, index);
        return Ptr._wrap(ret, Input);
    }

    /**
    * @param {Input} item
    * @returns {Promise<void>}
    */
    add(item) {
        const itemPtr = Ptr._assertClass(item, Input);
        item.ptr = null;
        return ChainLibs.inputsAdd(this.ptr, itemPtr);
    }
}

/**
* Type for representing a Transaction Output, composed of an Address and a Value
*/
export class Output extends Ptr {
    /**
    * @returns {Promise<Address>}
    */
    async address() {
        const ret = await ChainLibs.outputAddress(this.ptr);
        return Ptr._wrap(ret, Address);
    }

    /**
    * @returns {Promise<Value>}
    */
    async value() {
        const ret = await ChainLibs.outputValue(this.ptr);
        return Ptr._wrap(ret, Value);
    }
}

/**
*/
export class Outputs extends Ptr {
    /**
    * @returns {Promise<Outputs>}
    */
    static async new() {
        const ret = await ChainLibs.outputsNew();
        return Ptr._wrap(ret, Outputs);
    }
    
    /**
    * @returns {Promise<number>}
    */
    size() {
        return ChainLibs.outputsSize(this.ptr);
    }
    /**
    * @param {number} index
    * @returns {Promise<Output>}
    */
    async get(index) {
        const ret = await ChainLibs.outputsGet(this.ptr, index);
        return Ptr._wrap(ret, Ptr);
    }

    /**
    * @param {Input} item
    * @returns {Promise<void>}
    */
    add(item) {
        const itemPtr = Ptr._assertClass(item, Output);
        item.ptr = null;
        return ChainLibs.outputsAdd(this.ptr, itemPtr);
    }
}

/**
* Type for representing the hash of a Transaction, necessary for signing it
*/
export class TransactionSignDataHash extends Ptr {
    /**
    * @param {Uint8Array} bytes
    * @returns {Promise<TransactionSignDataHash>}
    */
    static async from_bytes(bytes) {
        const ret = await ChainLibs.transactionSignDataHashFromBytes(b64FromUint8Array(bytes));
        return Ptr._wrap(ret, TransactionSignDataHash);
    }

    /**
    * @param {string} input
    * @returns {Promise<TransactionSignDataHash>}
    */
    static async from_hex(input) {
        const ret = await ChainLibs.transactionSignDataHashFromHex(input);
        return Ptr._wrap(ret, TransactionSignDataHash);
    }

    /**
    * @returns {Promise<Uint8Array>}
    */
    async as_bytes() {
        const b64 = await ChainLibs.transactionSignDataHashAsBytes(this.ptr);
        return Uint8ArrayFromB64(b64);
    }
}

/**
* Type representing a unsigned transaction
*/
export class Transaction extends Ptr {
    /**
    * Get the transaction id, needed to compute its signature
    * @returns {Promise<TransactionSignDataHash>}
    */
    async id() {
        const ret = await ChainLibs.transactionId(this.ptr);
        return Ptr._wrap(ret, TransactionSignDataHash);
    }

    /**
    * Get collection of the inputs in the transaction (this allocates new copies of all the values)
    * @returns {Promise<Inputs>}
    */
    async inputs() {
        const ret = await ChainLibs.transactionInputs(this.ptr);
        return Ptr._wrap(ret, Inputs);
    }

    /**
    * Get collection of the outputs in the transaction (this allocates new copies of all the values)
    * @returns {Promise<Outputs>}
    */
    async outputs() {
        const ret = await ChainLibs.transactionOutputs(this.ptr);
        return Ptr._wrap(ret, Outputs);
    }
}

/**
*/
export class FragmentId extends Ptr {
    /**
    * @param {Uint8Array} bytes
    * @returns {Promise<FragmentId>}
    */
    static async calculate(bytes) {
        const ret = await ChainLibs.fragmentIdCalculate(b64FromUint8Array(bytes));
        return Ptr._wrap(ret, FragmentId);
    }

    /**
    * @returns {Promise<Uint8Array>}
    */
    async as_bytes() {
        const b64 = await ChainLibs.fragmentIdAsBytes(this.ptr);
        return Uint8ArrayFromB64(b64);
    }
}

/**
* Unspent transaction pointer. This is composed of:
* * the transaction identifier where the unspent output is (a FragmentId)
* * the output index within the pointed transaction\'s outputs
* * the value we expect to read from this output, this setting is added in order to protect undesired withdrawal
* and to set the actual fee in the transaction.
*/
export class UtxoPointer extends Ptr {
    /**
    * @param {FragmentId} fragmentId
    * @param {number} outputIndex
    * @param {Value} value
    * @returns {Promise<UtxoPointer>}
    */
    static async new(fragmentId, outputIndex, value) {
        const fragmentIdPtr = Ptr._assertClass(fragmentId, FragmentId);
        const valuePtr = Ptr._assertClass(value, Value);
        const ret = await ChainLibs.utxoPointerNew(fragmentIdPtr, outputIndex, valuePtr);
        return Ptr._wrap(ret, UtxoPointer);
    }
}

export class Fragment extends Ptr {
    /**
    * @param {Transaction} tx
    * @returns {Promise<Fragment>}
    */
    static async from_transaction(tx) {
        const txPtr = Ptr._assertClass(tx, Transaction);
        const ret = await ChainLibs.fragmentFromTransaction(txPtr);
        return Ptr._wrap(ret, Fragment);
    }

    /**
    * Get a Transaction if the Fragment represents one
    * @returns {Promise<Transaction>}
    */
    async get_transaction() {
        const ret = await ChainLibs.fragmentGetTransaction(this.ptr);
        return Ptr._wrap(ret, Transaction);
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
    is_pool_retirement() {
        return ChainLibs.fragmentIsPoolRetirement(this.ptr);
    }

    /**
    * @returns {Promise<boolean>}
    */
    is_pool_update() {
        return ChainLibs.fragmentIsPoolUpdate(this.ptr);
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
        return Ptr._wrap(ret, FragmentId);
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
        const ret = await ChainLibs.feeLinearFee(constantPtr, coefficientPtr, certificatePtr);
        return Ptr._wrap(ret, Fee);
    }

    /**
    * Compute the fee if possible (it can fail in case the values are out of range)
    * @param {Transaction} tx
    * @returns {Promise<Value>}
    */
    async calculate(tx) {
        const txPtr = Ptr._assertClass(tx, Transaction);
        const ret = await ChainLibs.feeCalculate(this.ptr, txPtr);
        return Ptr._wrap(ret, Value);
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
        const ret = await ChainLibs.outputPolicyOne(addressPtr);
        return Ptr._wrap(ret, OutputPolicy);
    }
}

/**
* Amount of the balance in the transaction.
*/
export class Balance extends Ptr {
    /**
    * @returns {Promise<boolean>}
    */
    is_positive() {
        return ChainLibs.balanceIsPositive(this.ptr);
    }

    /**
    * @returns {Promise<boolean>}
    */
    is_negative() {
        return ChainLibs.balanceIsNegative(this.ptr);
    }

    /**
    * @returns {Promise<boolean>}
    */
    is_zero() {
        return ChainLibs.balanceIsZero(this.ptr);
    }

    /**
    * Get value without taking into account if the balance is positive or negative
    * @returns {Promise<Value>}
    */
    async get_value() {
        const ret = await ChainLibs.balanceGetValue(this.ptr);
        return Ptr._wrap(ret, Value);
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

    /**
    * @returns {Promise<PublicKey>}
    */
    async to_public() {
        const ret = await ChainLibs.privateKeyToPublic(this.ptr);
        return Ptr._wrap(ret, PublicKey);
    }

    /**
    * @param {Uint8Array} bytes
    * @returns {Promise<PrivateKey>}
    */
    static async from_extended_bytes(bytes) {
        const ret = await ChainLibs.privateKeyFromExtendedBytes(b64FromUint8Array(bytes));
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

    /**
    * @param {number} counter
    * @returns {Promise<SpendingCounter>}
    */
    static async from_u32(counter) {
        const ret = await ChainLibs.spendingCounterFromU32(counter);
        return Ptr._wrap(ret, SpendingCounter);
    }
}

/**
*/
export class Bip32PublicKey extends Ptr {
    /**
    * derive this private key with the given index.
    *
    * # Security considerations
    *
    * * hard derivation index cannot be soft derived with the public key
    *
    * # Hard derivation vs Soft derivation
    *
    * If you pass an index below 0x80000000 then it is a soft derivation.
    * The advantage of soft derivation is that it is possible to derive the
    * public key too. I.e. derivation the private key with a soft derivation
    * index and then retrieving the associated public key is equivalent to
    * deriving the public key associated to the parent private key.
    *
    * Hard derivation index does not allow public key derivation.
    *
    * This is why deriving the private key should not fail while deriving
    * the public key may fail (if the derivation index is invalid).
    * @param {number} index
    * @returns {Promise<Bip32PublicKey>}
    */
    async derive(index) {
        const ret = await ChainLibs.bip32PublicKeyDerive(this.ptr, index);
        return Ptr._wrap(ret, Bip32PublicKey);
    }

    /**
    * @returns {Promise<PublicKey>}
    */
    async to_raw_key() {
        const ret = await ChainLibs.bip32PublicKeyToRawKey(this.ptr);
        return Ptr._wrap(ret, PublicKey);
    }

    /**
    * @param {Uint8Array} bytes
    * @returns {Promise<Bip32PublicKey>}
    */
    static async from_bytes(bytes) {
        const ret = await ChainLibs.bip32PublicKeyFromBytes(b64FromUint8Array(bytes));
        return Ptr._wrap(ret, Bip32PublicKey);
    }

    /**
    * @returns {Promise<Uint8Array>}
    */
    async as_bytes() {
        const b64 = await ChainLibs.bip32PublicKeyAsBytes(this.ptr);
        return Uint8ArrayFromB64(b64);
    }

    /**
    * @param {string} bech32Str
    * @returns {Promise<Bip32PublicKey>}
    */
    static async from_bech32(bech32Str) {
        const ret = await ChainLibs.bip32PublicKeyFromBech32(bech32Str);
        return Ptr._wrap(ret, Bip32PublicKey);
    }

    /**
    * @returns {Promise<string>}
    */
    to_bech32() {
        return ChainLibs.bip32PublicKeyToBech32(this.ptr);
    }
}

/**
*/
export class Bip32PrivateKey extends Ptr {
    /**
    * derive this private key with the given index.
    *
    * # Security considerations
    *
    * * hard derivation index cannot be soft derived with the public key
    *
    * # Hard derivation vs Soft derivation
    *
    * If you pass an index below 0x80000000 then it is a soft derivation.
    * The advantage of soft derivation is that it is possible to derive the
    * public key too. I.e. derivation the private key with a soft derivation
    * index and then retrieving the associated public key is equivalent to
    * deriving the public key associated to the parent private key.
    *
    * Hard derivation index does not allow public key derivation.
    *
    * This is why deriving the private key should not fail while deriving
    * the public key may fail (if the derivation index is invalid).
    * @param {number} index
    * @returns {Promise<Bip32PrivateKey>}
    */
    async derive(index) {
        const ret = await ChainLibs.bip32PrivateKeyDerive(this.ptr, index);
        return Ptr._wrap(ret, Bip32PrivateKey);
    }

    /**
    * @returns {Promise<Bip32PrivateKey>}
    */
    static async generate_ed25519_bip32() {
        const ret = await ChainLibs.bip32PrivateKeyGenerateEd25519Bip32();
        return Ptr._wrap(ret, Bip32PrivateKey);
    }

    /**
    * @returns {Promise<PrivateKey>}
    */
    async to_raw_key() {
        const ret = await ChainLibs.bip32PrivateKeyToRawKey(this.ptr);
        return Ptr._wrap(ret, PrivateKey);
    }

    /**
    * @returns {Promise<Bip32PublicKey>}
    */
    async to_public() {
        const ret = await ChainLibs.bip32PrivateKeyToPublic(this.ptr);
        return Ptr._wrap(ret, Bip32PublicKey);
    }

    /**
    * @param {Uint8Array} bytes
    * @returns {Promise<Bip32PrivateKey>}
    */
    static async from_bytes(bytes) {
        const ret = await ChainLibs.bip32PrivateKeyFromBytes(b64FromUint8Array(bytes));
        return Ptr._wrap(ret, Bip32PrivateKey);
    }

    /**
    * @returns {Promise<Uint8Array>}
    */
    async as_bytes() {
        const b64 = await ChainLibs.bip32PrivateKeyAsBytes(this.ptr);
        return Uint8ArrayFromB64(b64);
    }

    /**
    * @param {string} bech32Str
    * @returns {Promise<Bip32PrivateKey>}
    */
    static async from_bech32(bech32Str) {
        const ret = await ChainLibs.bip32PrivateKeyFromBech32(bech32Str);
        return Ptr._wrap(ret, Bip32PrivateKey);
    }

    /**
    * @returns {Promise<string>}
    */
    to_bech32() {
        return ChainLibs.bip32PrivateKeyToBech32(this.ptr);
    }

    /**
    * @param {Uint8Array} entropy
    * @param {Uint8Array} password
    * @returns {Promise<Bip32PrivateKey>}
    */
    static async from_bip39_entropy(entropy, password) {
        const ret = await ChainLibs.bip32PrivateKeyFromBip39Entropy(b64FromUint8Array(entropy), b64FromUint8Array(password));
        return Ptr._wrap(ret, Bip32PrivateKey);
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
    * Generate Witness for an utxo-based transaction Input
    * @param {Hash} genesisHash
    * @param {TransactionSignDataHash} transactionId
    * @param {PrivateKey} secretKey
    * @returns {Promise<Witness>}
    */
    static async for_utxo(genesisHash, transactionId, secretKey) {
        const genesisHashPtr = Ptr._assertClass(genesisHash, Hash);
        const transactionIdPtr = Ptr._assertClass(transactionId, TransactionSignDataHash);
        const secretKeyPtr = Ptr._assertClass(secretKey, PrivateKey);
        const ret = await ChainLibs.witnessForUtxo(genesisHashPtr, transactionIdPtr, secretKeyPtr);
        return Ptr._wrap(ret, Witness);
    }

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
        const transactionIdPtr = Ptr._assertClass(transactionId, TransactionSignDataHash);
        const secretKeyPtr = Ptr._assertClass(secretKey, PrivateKey);
        const accountSpendingCounterPtr = Ptr._assertClass(accountSpendingCounter, SpendingCounter);
        const ret = await ChainLibs.witnessForAccount(genesisHashPtr, transactionIdPtr, secretKeyPtr, accountSpendingCounterPtr);
        return Ptr._wrap(ret, Witness);
    }

    /**
    * Generate Witness for an utxo-based transaction Input
    * @param {Hash} genesisHash
    * @param {TransactionSignDataHash} transactionId
    * @param {Bip32PrivateKey} secretKey
    * @returns {Promise<Witness>}
    */
    static async for_legacy_icarus_utxo(genesisHash, transactionId, secretKey) {
        const genesisHashPtr = Ptr._assertClass(genesisHash, Hash);
        const transactionIdPtr = Ptr._assertClass(transactionId, TransactionSignDataHash);
        const secretKeyPtr = Ptr._assertClass(secretKey, Bip32PrivateKey);
        const ret = await ChainLibs.witnessForLegacyIcarusUtxo(genesisHashPtr, transactionIdPtr, secretKeyPtr);
        return Ptr._wrap(ret, Witness);
    }
}

/**
*/
export class Witnesses extends Ptr {
    /**
    * @returns {Promise<Witnesses>}
    */
    static async new() {
        const ret = await ChainLibs.witnessesNew();
        return Ptr._wrap(ret, Witnesses);
    }

    /**
    * @returns {Promise<number>}
    */
    size() {
        return ChainLibs.witnessesSize(this.ptr);
    }

    /**
    * @param {number} index
    * @returns {Promise<Witness>}
    */
    async get(index) {
        const ret = await ChainLibs.witnessesGet(this.ptr, index);
        return Ptr._wrap(ret, Witness);
    }

    /**
    * @param {Witness} item
    * @returns {Promise<void>}
    */
    add(item) {
        const itemPtr = Ptr._assertClass(item, Witness);
        item.ptr = null;
        return ChainLibs.witnessesAdd(this.ptr, itemPtr);
    }
}

/**
*/
export class PayloadAuthData extends Ptr {
    /**
    * @returns {Promise<PayloadAuthData>}
    */
    static async for_no_payload() {
        const ret = await ChainLibs.payloadAuthDataForNoPayload();
        return Ptr._wrap(ret, PayloadAuthData);
    }
}

/**
*/
export class TransactionBindingAuthData extends Ptr {
}

/**
*/
export class TransactionBuilderSetAuthData extends Ptr {
    /**
    * @returns {Promise<TransactionBindingAuthData>}
    */
    async get_auth_data() {
        const ret = await ChainLibs.transactionBuilderSetAuthDataGetAuthData(this.ptr);
        return Ptr._wrap(ret, TransactionBindingAuthData);
    }

    /**
    * Set the authenticated data
    * @param {PayloadAuthData} auth
    * @returns {Promise<Transaction>}
    */
    async set_payload_auth(auth) {
        const authPtr = Ptr._assertClass(auth, PayloadAuthData);
        const ptr = this.ptr;
        this.ptr = null;
        const ret = await ChainLibs.transactionBuilderSetAuthDataSetPayloadAuth(ptr, authPtr);
        return Ptr._wrap(ret, Transaction);
    }
}

/**
*/
export class TransactionBuilderSetWitness extends Ptr {
    /**
    * @returns {Promise<TransactionSignDataHash>}
    */
    async get_auth_data_for_witness() {
        const ret = await ChainLibs.transactionBuilderSetWitnessGetAuthDataForWitness(this.ptr);
        return Ptr._wrap(ret, TransactionSignDataHash);
    }

    /**
    * @param {Witnesses} witnesses
    * @returns {Promise<TransactionBuilderSetAuthData>}
    */
    async set_witnesses(witnesses) {
        const witnessesPtr = Ptr._assertClass(witnesses, Ptr);
        const ptr = this.ptr;
        this.ptr = null;
        const ret = await ChainLibs.transactionBuilderSetWitnessSetWitnesses(ptr, witnessesPtr);
        return Ptr._wrap(ret, TransactionBuilderSetAuthData);
    }
}

/**
*/
export class TransactionBuilderSetIOs extends Ptr {
    /**
    * @param {Inputs} inputs
    * @param {Outputs} outputs
    * @returns {Promise<TransactionBuilderSetWitness>}
    */
    async set_ios(inputs, outputs) {
        const inputsPtr = Ptr._assertClass(inputs, Inputs);
        const outputsPtr = Ptr._assertClass(outputs, Outputs);
        const ptr = this.ptr;
        this.ptr = null;
        const ret = await ChainLibs.transactionBuilderSetIOsSetIOs(ptr, inputsPtr, outputsPtr);
        return Ptr._wrap(ret, TransactionBuilderSetWitness);
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
    static async new() {
        const ret = await ChainLibs.transactionBuilderNew();
        return Ptr._wrap(ret, TransactionBuilder);
    }

    /**
    * @param {Certificate} cert
    * @returns {Promise<TransactionBuilderSetIOs>}
    */
    async payload(cert) {
        const certPtr = Ptr._assertClass(cert, Certificate);
        const ptr = this.ptr;
        this.ptr = null;
        const ret = await ChainLibs.transactionBuilderPayload(ptr, certPtr);
        return Ptr._wrap(ret, TransactionBuilderSetIOs);
    }

    /**
    * @returns {Promise<TransactionBuilderSetIOs>}
    */
    async no_payload() {
        const ptr = this.ptr;
        this.ptr = null;
        const ret = await ChainLibs.transactionBuilderNoPayload(ptr);
        return Ptr._wrap(ret, TransactionBuilderSetIOs);
    }
}

/**
*/
export class Payload extends Ptr {
    /**
    * @returns {Promise<Payload>}
    */
    static async no_payload() {
        const ret = await ChainLibs.payloadNoPayload();
        return Ptr._wrap(ret, Payload);
    }
}

/**
*/
export class InputOutput extends Ptr {
    /**
    * @returns {Promise<Inputs>}
    */
    async inputs() {
        const ret = await ChainLibs.inputOutputInputs(this.ptr);
        return Ptr._wrap(ret, Inputs);
    }
    /**
    * @returns {Promise<Outputs>}
    */
    async outputs() {
        const ret = await ChainLibs.inputOutputOutputs(this.ptr);
        return Ptr._wrap(ret, Outputs);
    }
}

/**
*/
export class InputOutputBuilder extends Ptr {
    /**
    * @returns {Promise<InputOutputBuilder>}
    */
    static async empty() {
        const ret = await ChainLibs.inputOutputBuilderEmpty();
        return Ptr._wrap(ret, InputOutputBuilder);
    }

    /**
    * Add input to the IO Builder
    * @param {Input} input
    * @returns {Promise<void>}
    */
    add_input(input) {
        const inputPtr = Ptr._assertClass(input, Input);
        return ChainLibs.inputOutputBuilderAddInput(this.ptr, inputPtr);
    }

    /**
    * Add output to the IO Builder
    * @param {Address} address
    * @param {Value} value
    * @returns {Promise<void>}
    */
    add_output(address, value) {
        const addressPtr = Ptr._assertClass(address, Address);
        const valuePtr = Ptr._assertClass(value, Value);
        return ChainLibs.inputOutputBuilderAddOutput(this.ptr, addressPtr, valuePtr);
    }

    /**
    * Estimate fee with the currently added inputs, outputs and certificate based on the given algorithm
    * @param {Fee} fee
    * @param {Payload} payload
    * @returns {Value}
    */
    async estimate_fee(fee, payload) {
        const feePtr = Ptr._assertClass(fee, Fee);
        const payloadPtr = Ptr._assertClass(payload, Payload);
        const ret = await ChainLibs.inputOutputBuilderEstimateFee(this.ptr, feePtr, payloadPtr);
        return Ptr._wrap(ret, Value);
    }

    /**
    * @returns {Promise<InputOutput>}
    */
    async build() {
        const ptr = this.ptr;
        this.ptr = null;
        const ret = await ChainLibs.inputOutputBuilderBuild(ptr);
        return Ptr._wrap(ret, InputOutput);
    }

    /**
    * Seal the transaction by passing fee rule and the output policy
    * @param {Payload} payload
    * @param {Fee} feeAlgorithm
    * @param {OutputPolicy} policy
    * @returns {Promise<InputOutput>}
    */
    async seal_with_output_policy(payload, feeAlgorithm, policy) {
        const payloadPtr = Ptr._assertClass(payload, Payload);
        const feeAlgorithmPtr = Ptr._assertClass(feeAlgorithm, Fee);
        const policyPtr = Ptr._assertClass(policy, OutputPolicy);
        const ptr = this.ptr;
        this.ptr = null;
        const ret = await ChainLibs.inputOutputBuilderSealWithOutputPolicy(ptr, payloadPtr, feeAlgorithmPtr, policyPtr);
        return Ptr._wrap(ret, InputOutput);
    }
}

/**
*/
export class StakeDelegationAuthData extends Ptr {
    /**
    * @param {AccountBindingSignature} signature
    * @returns {Promise<StakeDelegationAuthData>}
    */
    static async new(signature) {
        const signaturePtr = Ptr._assertClass(signature, Ptr);
        const ret = await ChainLibs.stakeDelegationAuthDataNew(signaturePtr);
        return Ptr._wrap(ret, StakeDelegationAuthData);
    }
}

/**
*/
export class PoolId extends Ptr {}

/**
* Delegation Ratio type express a number of parts
* and a list of pools and their individual parts
*
* E.g. parts: 7, pools: [(A,2), (B,1), (C,4)] means that
* A is associated with 2/7 of the stake, B has 1/7 of stake and C
* has 4/7 of the stake.
*
* It\'s invalid to have less than 2 elements in the array,
* and by extension parts need to be equal to the sum of individual
* pools parts.
*/
export class DelegationRatio extends Ptr {}

/**
* Set the choice of delegation:
*
* * No delegation
* * Full delegation of this account to a specific pool
* * Ratio of stake to multiple pools
*/
export class DelegationType extends Ptr {
    /**
    * @returns {Promise<DelegationType>}
    */
    static async non_delegated() {
        const ret = await ChainLibs.delegationTypeNonDelegated();
        return Ptr._wrap(ret, DelegationType);
    }

    /**
    * @param {PoolId} poolId
    * @returns {Promise<DelegationType>}
    */
    static async full(poolId) {
        const poolIdPtr = Ptr._assertClass(poolId, PoolId);
        const ret = await ChainLibs.delegationTypeFull(poolIdPtr);
        return Ptr._wrap(ret, DelegationType);
    }

    /**
    * @param {DelegationRatio} r
    * @returns {Promise<DelegationType>}
    */
    static async ratio(r) {
        const rPtr = Ptr._assertClass(r, DelegationRatio);
        const ret = await ChainLibs.delegationTypeRatio(rPtr);
        return Ptr._wrap(ret, DelegationType);
    }

    /**
    * @returns {Promise<number>}
    */
    async get_kind() {
        const ret = await ChainLibs.delegationTypeGetKind(this.ptr);
        return ret;
    }
    
    /**
    * @returns {Promise<PoolId | undefined>}
    */
    async get_full() {
        const ret = await ChainLibs.delegationTypeGetFull(this.ptr);
        return Ptr._wrap(ret, PoolId);
    }
}

/**
*/
export class AccountIdentifier extends Ptr {}

/**
*/
export class StakeDelegation extends Ptr {
    /**
    * Create a stake delegation object from account (stake key) to pool_id
    * @param {DelegationType} delegationType
    * @param {PublicKey} account
    * @returns {Promise<StakeDelegation>}
    */
    static async new(delegationType, account) {
        const delegationTypePtr = Ptr._assertClass(delegationType, Ptr);
        const accountPtr = Ptr._assertClass(account, PublicKey);
        const ret = await ChainLibs.stakeDelegationNew(delegationTypePtr, accountPtr);
        return Ptr._wrap(ret, StakeDelegation);
    }

    /**
    * @returns {Promise<DelegationType>}
    */
    async delegation_type() {
        const ret = await ChainLibs.stakeDelegationDelegationType(this.ptr);
        return Ptr._wrap(ret, Ptr);
    }

    /**
    * @returns {Promise<AccountIdentifier>}
    */
    async account() {
        const ret = await ChainLibs.stakeDelegationAccount(this.ptr);
        return Ptr._wrap(ret, AccountIdentifier);
    }

    /**
    * @returns {Promise<Uint8Array>}
    */
    async as_bytes() {
        const b64 = await ChainLibs.stakeDelegationAsBytes(this.ptr);
        return Uint8ArrayFromB64(b64);
    }

    /**
    * @param {Uint8Array} bytes
    * @returns {Promise<StakeDelegation>}
    */
    static async from_bytes(bytes) {
        const ret = await ChainLibs.stakeDelegationFromBytes(b64FromUint8Array(bytes));
        return Ptr._wrap(ret, StakeDelegation);
    }
}

/**
*/
export class PoolRegistration extends Ptr {}

/**
*/
export class PoolRetirement extends Ptr {}

/**
*/
export class OwnerStakeDelegation extends Ptr {}

/**
*/
export class Certificate extends Ptr {
    /**
    * Create a Certificate for StakeDelegation
    * @param {StakeDelegation} stakeDelegation
    * @returns {Promise<Certificate>}
    */
    static async stake_delegation(stakeDelegation) {
        const stakeDelegationPtr = Ptr._assertClass(stakeDelegation, StakeDelegation);
        const ret = await ChainLibs.certificateStakeDelegation(stakeDelegationPtr);
        return Ptr._wrap(ret, Certificate);
    }

    /**
    * Create a Certificate for PoolRegistration
    * @param {PoolRegistration} poolRegistration
    * @returns {Promise<Certificate>}
    */
    static async stake_pool_registration(poolRegistration) {
        const poolRegistrationPtr = Ptr._assertClass(poolRegistration, PoolRegistration);
        const ret = await ChainLibs.certificateStakePoolRegistration(poolRegistrationPtr);
        return Ptr._wrap(ret, Certificate);
    }

    /**
    * Create a Certificate for PoolRetirement
    * @param {PoolRetirement} poolRetirement
    * @returns {Promise<Certificate>}
    */
    static async stake_pool_retirement(poolRetirement) {
        const poolRetirementPtr = Ptr._assertClass(poolRetirement, PoolRetirement);
        const ret = await ChainLibs.certificateStakePoolRetirement(poolRetirementPtr);
        return Ptr._wrap(ret, Certificate);
    }

    /**
    * @returns {Promise<number>}
    */
    get_type() {
        return ChainLibs.certificateGetType(this.ptr);
    }

    /**
    * @returns {Promise<StakeDelegation>}
    */
    async get_stake_delegation() {
        const ret = await ChainLibs.certificateGetStakeDelegation(this.ptr);
        return Ptr._wrap(ret, StakeDelegation);
    }

    /**
    * @returns {Promise<OwnerStakeDelegation>}
    */
    async get_owner_stake_delegation() {
        const ret = await ChainLibs.certificateGetOwnerStakeDelegation(this.ptr);
        return Ptr._wrap(ret, OwnerStakeDelegation);
    }

    /**
    * @returns {Promise<PoolRegistration>}
    */
    async get_pool_registration() {
        const ret = await ChainLibs.certificateGetPoolRegistration(this.ptr);
        return Ptr._wrap(ret, PoolRegistration);
    }

    /**
    * @returns {Promise<PoolRetirement>}
    */
    async get_pool_retirement() {
        const ret = await ChainLibs.certificateGetPoolRetirement(this.ptr);
        return Ptr._wrap(ret, PoolRetirement);
    }
}

/**
*/
export class AccountBindingSignature extends Ptr {
    /**
    * @param {PrivateKey} privateKey
    * @param {TransactionBindingAuthData} authData
    * @returns {Promise<AccountBindingSignature>}
    */
    static async new_single(privateKey, authData) {
        const privateKeyPtr = Ptr._assertClass(privateKey, PrivateKey);
        const authDataPtr = Ptr._assertClass(authData, TransactionBindingAuthData);
        const ret = await ChainLibs.accountBindingSignatureNewSingle(privateKeyPtr, authDataPtr);
        return Ptr._wrap(ret, AccountBindingSignature);
    }
}
