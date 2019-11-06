export enum AddressDiscrimination {
  Production,
  Test,
}

export class Ptr {
  /**
    * Frees the pointer
    * @returns {Promise<void>}
    */
  free(): Promise<void>;
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
  * const address = Address.from_string(&#39;ca1q09u0nxmnfg7af8ycuygx57p5xgzmnmgtaeer9xun7hly6mlgt3pjyknplu&#39;);
  * ```
  * @param {string} s
  * @returns {Promise<Address>}
  */
  static from_string(s: string): Promise<Address>;

  /**
  * Get Address bech32 (string) representation with a given prefix
  * ```javascript
  * let public_key = PublicKey.from_bech32(
  *     &#39;ed25519_pk1kj8yvfrh5tg7n62kdcw3kw6zvtcafgckz4z9s6vc608pzt7exzys4s9gs8&#39;
  * );
  * let discriminant = AddressDiscrimination.Test;
  * let address = Address.single_from_public_key(public_key, discriminant);
  * address.to_string(&#39;ta&#39;)
  * // ta1sj6gu33yw73dr60f2ehp6xemgf30r49rzc25gkrfnrfuuyf0mycgnj78ende550w5njvwzyr20q6rypdea597uu3jnwfltljddl59cseaq7yn9
  * ```
  * @param {string} prefix
  * @returns {Promise<string>}
  */
  to_string(prefix: string): Promise<string>;

  /**
  * Construct a single non-account address from a public key
  * ```javascript
  * let public_key = PublicKey.from_bech32(
  *     &#39;ed25519_pk1kj8yvfrh5tg7n62kdcw3kw6zvtcafgckz4z9s6vc608pzt7exzys4s9gs8&#39;
  * );
  * let address = Address.single_from_public_key(public_key, AddressDiscrimination.Test);
  * ```
  * @param {PublicKey} key
  * @param {number} discrimination
  * @returns {Promise<Address>}
  */
  static single_from_public_key(key: PublicKey, discrimination: number): Promise<Address>;

  /**
  * Construct a non-account address from a pair of public keys, delegating founds from the first to the second
  * @param {PublicKey} key
  * @param {PublicKey} delegation
  * @param {number} discrimination
  * @returns {Promise<Address>}
  */
  static delegation_from_public_key(key: PublicKey, delegation: PublicKey, discrimination: number): Promise<Address>;

  /**
  * Construct address of account type from a public key
  * @param {PublicKey} key
  * @param {number} discrimination
  * @returns {Promise<Address>}
  */
  static account_from_public_key(key: PublicKey, discrimination: number): Promise<Address>;
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
/**
* This is either an single account or a multisig account depending on the witness type
*/
export class Account extends Ptr {
  /**
  * @param {Address} address
  * @returns {Promise<Account>}
  */
  static from_address(address: Address): Promise<Account>;
}

/**
*/
export class Input extends Ptr {
  /**
  * @param {Account} account
  * @param {Value} v
  * @returns {Promise<Input>}
  */
  static from_account(account: Account, v: Value): Promise<Input>;

  /**
  * @returns {Promise<Value>} 
  */
  value(): Promise<Value>;
}

/**
*/
export class Inputs extends Ptr {
  /**
  * @returns {Promise<number>}
  */
  size(): Promise<number>;
  /**
  * @param {number} index
  * @returns {Promise<Input>}
  */
  get(index: number): Promise<Input>;
}

/**
* Type for representing a Transaction Output, composed of an Address and a Value
*/
export class Output extends Ptr {
  /**
  * @returns {Promise<Address>}
  */
  address(): Promise<Address>;

  /**
  * @returns {Promise<Value>}
  */
  value(): Promise<Value>;
}

/**
*/
export class Outputs extends Ptr {
  /**
  * @returns {Promise<number>}
  */
  size(): Promise<number>;
  /**
  * @param {number} index
  * @returns {Promise<Output>}
  */
  get(index: number): Promise<Output>;
}

/**
* Type for representing the hash of a Transaction, necessary for signing it
*/
export class TransactionSignDataHash extends Ptr {
  /**
  * @param {Uint8Array} bytes
  * @returns {Promise<TransactionSignDataHash>}
  */
  static from_bytes(bytes: Uint8Array): Promise<TransactionSignDataHash>;

  /**
  * @param {string} input
  * @returns {Promise<TransactionSignDataHash>}
  */
  static from_hex(input: string): Promise<TransactionSignDataHash>;

  /**
  * @returns {Promise<Uint8Array>}
  */
  as_bytes(): Promise<Uint8Array>;
}

/**
* Type representing a unsigned transaction
*/
export class Transaction extends Ptr {
  /**
  * Get the transaction id, needed to compute its signature
  * @returns {Promise<TransactionSignDataHash>}
  */
  id(): Promise<TransactionSignDataHash>;

  /**
  * Get collection of the inputs in the transaction (this allocates new copies of all the values)
  * @returns {Promise<Inputs>}
  */
  inputs(): Promise<Inputs>;

  /**
  * Get collection of the outputs in the transaction (this allocates new copies of all the values)
  * @returns {Promise<Outputs>}
  */
  outputs(): Promise<Outputs>;
}

/**
* Type for representing a Transaction with Witnesses (signatures)
*/
export class AuthenticatedTransaction extends Ptr {
  /**
  * Get a copy of the inner Transaction, discarding the signatures
  * @returns {Promise<Transaction>}
  */
  transaction(): Promise<Transaction>;
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
  static linear_fee(constant: Value, coefficient: Value, certificate: Value): Promise<Fee>;

  /**
  * Compute the fee if possible (it can fail in case the values are out of range)
  * @param {Transaction} tx
  * @returns {Promise<Value>}
  */
  calculate(tx: Transaction): Promise<Value>;
}

/**
*/
export class FragmentId extends Ptr {
  /**
  * @param {Uint8Array} bytes
  * @returns {Promise<FragmentId>}
  */
  static from_bytes(bytes: Uint8Array): Promise<FragmentId>;

  /**
  * @returns {Promise<Uint8Array>}
  */
  as_bytes(): Promise<Uint8Array>;
}

/**
* All possible messages recordable in the Block content
*/
export class Fragment extends Ptr {
  /**
  * @param {AuthenticatedTransaction} tx
  * @returns {Promise<Fragment>}
  */
  static from_authenticated_transaction(tx: AuthenticatedTransaction): Promise<Fragment>;

  /**
  * Deprecated: Use `from_authenticated_transaction` instead
  * @param {AuthenticatedTransaction} tx
  * @returns {Promise<Fragment>}
  */
  static from_generated_transaction(tx: AuthenticatedTransaction): Promise<Fragment>;

  /**
  * Get a Transaction if the Fragment represents one
  * @returns {Promise<AuthenticatedTransaction>}
  */
  get_transaction(): Promise<AuthenticatedTransaction>;

  /**
  * @returns {Promise<Uint8Array>}
  */
  as_bytes(): Promise<Uint8Array>;

  /**
  * @returns {Promise<boolean>}
  */
  is_initial(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>}
  */
  is_transaction(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>}
  */
  is_owner_stake_delegation(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>}
  */
  is_stake_delegation(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>}
  */
  is_pool_registration(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>}
  */
  is_pool_management(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>}
  */
  is_old_utxo_declaration(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>}
  */
  is_update_proposal(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>}
  */
  is_update_vote(): Promise<boolean>;

  /**
  * @returns {Promise<FragmentId>}
  */
  id(): Promise<FragmentId>;
}

/**
* Type for representing a generic Hash
*/
export class Hash extends Ptr {
  /**
  * @param {string} hex_string
  * @returns {Promise<Hash>}
  */
  static from_hex(hex_string: string): Promise<Hash>;
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
  static forget(): Promise<OutputPolicy>;

  /**
  * use the given address as the only change address
  * @param {Address} address
  * @returns {Promise<OutputPolicy>}
  */
  static one(address: Address): Promise<OutputPolicy>;
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
  static from_bech32(bech32_str: string): Promise<PrivateKey>;

  /**
  * @returns {Promise<PublicKey>}
  */
  async to_public(): Promise<PublicKey>
}

/**
* ED25519 key used as public key
*/
export class PublicKey extends Ptr {
  /**
  * Get private key from its bech32 representation
  * Example:
  * ```javascript
  * const pkey = PublicKey.from_bech32(&#39;ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2&#39;);
  * ```
  * @param {string} bech32_str
  * @returns {Promise<PublicKey>}
  */
  static from_bech32(bech32_str: string): Promise<PublicKey>;

  /**
  * @returns {Promise<Uint8Array>}
  */
  as_bytes(): Promise<Uint8Array>;
}

/**
*/
export class SpendingCounter extends Ptr {
  /**
  * @returns {Promise<SpendingCounter>}
  */
  static zero(): Promise<SpendingCounter>;

  /**
  * @param {number} counter 
  * @returns {Promise<SpendingCounter>} 
  */
  static from_u32(counter: number): Promise<SpendingCounter>;
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
  static new_no_payload(): Promise<TransactionBuilder>;

  /**
  * Add input to the transaction
  * @param {Input} input
  * @returns {Promise<void>}
  */
  add_input(input: Input): Promise<void>;

  /**
  * Add output to the transaction
  * @param {Address} address
  * @param {Value} value
  * @returns {Promise<void>}
  */
  add_output(address: Address, value: Value): Promise<void>;

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
  * @param {OutputPolicy} output_policy
  * @returns {Promise<Transaction>}
  */
  seal_with_output_policy(fee: Fee, output_policy: OutputPolicy): Promise<Transaction>;
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
  static new(transaction: Transaction): Promise<TransactionFinalizer>;

  /**
  * Set the witness for the corresponding index, the index corresponds to the order in which the inputs were added to the transaction
  * @param {number} index
  * @param {Witness} witness
  * @returns {Promise<void>}
  */
  set_witness(index: number, witness: Witness): Promise<void>;

  /**
  * @returns {Promise<TransactionSignDataHash>}
  */
  get_tx_sign_data_hash(): Promise<TransactionSignDataHash>;

  /**
  * Deprecated: Use `get_tx_sign_data_hash` instead\
  * @returns {Promise<AuthenticatedTransaction>}
  */
  build(): Promise<AuthenticatedTransaction>;
}

/**
* Type used for representing certain amount of lovelaces.
* It wraps an unsigned 64 bits number.
* Strings are used for passing to and from javascript,
* as the native javascript Number type can\'t hold the entire u64 range
* and BigInt is not yet implemented in all the browsers
*/
export class Value extends Ptr {
  /**
  * Parse the given string into a rust u64 numeric type.
  * @param {string} s
  * @returns {Promise<Value>}
  */
  static from_str(s: string): Promise<Value>;

  /**
  * Return the wrapped u64 formatted as a string.
  * @returns {Promise<string>}
  */
  to_str(): Promise<string>;

  /**
  * @param {Value} other
  * @returns {Promise<Value>}
  */
  checked_add(other: Value): Promise<Value>;

  /**
  * @param {Value} other
  * @returns {Promise<Value>}
  */
  checked_sub(other: Value): Promise<Value>;
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
  * @param {Hash} genesis_hash
  * @param {TransactionSignDataHash} transaction_id
  * @param {PrivateKey} secret_key
  * @param {SpendingCounter} account_spending_counter
  * @returns {Promise<Witness>}
  */
  static for_account(genesis_hash: Hash, transaction_id: TransactionSignDataHash, secret_key: PrivateKey, account_spending_counter: SpendingCounter): Promise<Witness>;
}
