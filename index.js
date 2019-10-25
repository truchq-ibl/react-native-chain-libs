import { NativeModules } from 'react-native';

const { ChainLibs } = NativeModules;

function Uint8ArrayFromB64(base64_string) {
    return Uint8Array.from(atob(base64_string), c => c.charCodeAt(0));
}

class Ptr {
    static _wrap(ptr, klass) {
        const obj = Object.create(klass.prototype);
        obj.ptr = ptr;
        return obj;
    }

    _assertClass(klass) {
        if (!(this instanceof klass)) {
            throw new Error(`expected instance of ${klass.name}`);
        }
        return this.ptr;
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
export const AddressDiscrimination = Object.freeze({ Production:0, Test:1 });

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
        const ret = await ChainLibs.valueCheckedAdd(this.ptr, other._assertClass(Value));
        return Ptr._wrap(ret, Value);
    }
    /**
    * @param {Value} other
    * @returns {Promise<Value>}
    */
    async checked_sub(other) {
        const ret = await ChainLibs.valueCheckedSub(this.ptr, other._assertClass(Value));
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
        return Ptr.__wrap(ret, PublicKey);
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
        return Ptr.__wrap(ret, Address);
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
        const ret = await ChainLibs.addressSingleFromPublicKey(key._assertClass(PublicKey), discrimination);
        return Ptr.__wrap(ret, Address);
    }
    // /**
    // * Construct a non-account address from a pair of public keys, delegating founds from the first to the second
    // * @param {PublicKey} key
    // * @param {PublicKey} delegation
    // * @param {number} discrimination
    // * @returns {Address}
    // */
    // static delegation_from_public_key(key, delegation, discrimination) {
    //     _assertClass(key, PublicKey);
    //     const ptr0 = key.ptr;
    //     key.ptr = 0;
    //     _assertClass(delegation, PublicKey);
    //     const ptr1 = delegation.ptr;
    //     delegation.ptr = 0;
    //     const ret = wasm.address_delegation_from_public_key(ptr0, ptr1, discrimination);
    //     return Address.__wrap(ret);
    // }
    // /**
    // * Construct address of account type from a public key
    // * @param {PublicKey} key
    // * @param {number} discrimination
    // * @returns {Address}
    // */
    // static account_from_public_key(key, discrimination) {
    //     _assertClass(key, PublicKey);
    //     const ptr0 = key.ptr;
    //     key.ptr = 0;
    //     const ret = wasm.address_account_from_public_key(ptr0, discrimination);
    //     return Address.__wrap(ret);
    // }
}
