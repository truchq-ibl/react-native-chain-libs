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
export enum AddressDiscrimination {
  Production,
  Test,
}

/**
*/
export enum AddressKind {
  Single,
  Group,
  Account,
  Multisig,
}

/**
*/
export enum CertificateKind {
  StakeDelegation,
  OwnerStakeDelegation,
  PoolRegistration,
  PoolRetirement,
  PoolUpdate,
}

export type Optional<T> = T | undefined;

export class Ptr {
  /**
    * Frees the pointer
    * @returns {Promise<void>}
    */
  free(): Promise<void>;
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
  static from_bytes(bytes: Uint8Array): Promise<Address>;
  
  /**
  * @returns {Promise<Uint8Array>} 
  */
  as_bytes(): Promise<Uint8Array>;

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
  * @param {AddressDiscrimination} discrimination
  * @returns {Promise<Address>}
  */
  static single_from_public_key(key: PublicKey, discrimination: AddressDiscrimination): Promise<Address>;

  /**
  * Construct a non-account address from a pair of public keys, delegating founds from the first to the second
  * @param {PublicKey} key
  * @param {PublicKey} delegation
  * @param {AddressDiscrimination} discrimination
  * @returns {Promise<Address>}
  */
  static delegation_from_public_key(key: PublicKey, delegation: PublicKey, discrimination: AddressDiscrimination): Promise<Address>;

  /**
  * Construct address of account type from a public key
  * @param {PublicKey} key
  * @param {AddressDiscrimination} discrimination
  * @returns {Promise<Address>}
  */
  static account_from_public_key(key: PublicKey, discrimination: AddressDiscrimination): Promise<Address>;

  /**
  * @returns {Promise<AddressDiscrimination>} 
  */
  get_discrimination(): Promise<AddressDiscrimination>;

  /**
  * @returns {Promise<AddressKind>} 
  */
  get_kind(): Promise<AddressKind>;

  /**
  * @returns {Promise<Optional<SingleAddress>>} 
  */
  to_single_address(): Promise<Optional<SingleAddress>>;

  /**
  * @returns {Promise<Option<GroupAddress>>} 
  */
  to_group_address(): Promise<Optional<GroupAddress>>;

  /**
  * @returns {Promise<Optional<AccountAddress>>} 
  */
  to_account_address(): Promise<Optional<AccountAddress>>;
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

  /**
  * @param {PublicKey} key 
  * @returns {Promise<Account>} 
  */
  static single_from_public_key(key: PublicKey): Promise<Account>;
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
  * @returns {Promise<Inputs>} 
  */
  static new(): Promise<Inputs>;

  /**
  * @returns {Promise<number>}
  */
  size(): Promise<number>;

  /**
  * @param {number} index
  * @returns {Promise<Input>}
  */
  get(index: number): Promise<Input>;

  /**
  * @param {Input} item 
  */
  add(item: Input): Promise<void>;
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
  * @returns {Promise<Outputs>} 
  */
  static new(): Promise<Outputs>;

  /**
  * @returns {Promise<number>}
  */
  size(): Promise<number>;

  /**
  * @param {number} index
  * @returns {Promise<Output>}
  */
  get(index: number): Promise<Output>;

  /**
  * @param {Output} item 
  * @returns {Promise<void>}
  */
  add(item: Output): Promise<void>;
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
  static calculate(bytes: Uint8Array): Promise<FragmentId>;

  /**
  * @returns {Promise<Uint8Array>}
  */
  as_bytes(): Promise<Uint8Array>;
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
  * @param {FragmentId} fragment_id 
  * @param {number} output_index 
  * @param {Value} value 
  * @returns {Promise<UtxoPointer>} 
  */
  static new(fragment_id: FragmentId, output_index: number, value: Value): Promise<UtxoPointer>;
}

/**
* All possible messages recordable in the Block content
*/
export class Fragment extends Ptr {
  /**
  * @param {Transaction} tx
  * @returns {Promise<Fragment>}
  */
  static from_transaction(tx: Transaction): Promise<Fragment>;

  /**
  * Get a Transaction if the Fragment represents one
  * @returns {Promise<Transaction>}
  */
  get_transaction(): Promise<Transaction>;

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
  is_pool_retirement(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>}
  */
  is_pool_update(): Promise<boolean>;

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
  to_public(): Promise<PublicKey>;

  /**
  * @param {Uint8Array} bytes 
  * @returns {Promise<PrivateKey>} 
  */
  static from_extended_bytes(bytes: Uint8Array): Promise<PrivateKey>;
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
* Amount of the balance in the transaction.
*/
export class Balance extends Ptr {
  /**
  * @returns {Promise<boolean>} 
  */
  is_positive(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>} 
  */
  is_negative(): Promise<boolean>;

  /**
  * @returns {Promise<boolean>} 
  */
  is_zero(): Promise<boolean>;

  /**
  * Get value without taking into account if the balance is positive or negative
  * @returns {Promise<Value>} 
  */
  get_value(): Promise<Value>;
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
  * Generate Witness for an utxo-based transaction Input
  * @param {Hash} genesis_hash 
  * @param {TransactionSignDataHash} transaction_id 
  * @param {PrivateKey} secret_key 
  * @returns {Promise<Witness>} 
  */
  static for_utxo(genesis_hash: Hash, transaction_id: TransactionSignDataHash, secret_key: PrivateKey): Promise<Witness>;

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

  /**
  * Generate Witness for an utxo-based transaction Input
  * @param {Hash} genesis_hash 
  * @param {TransactionSignDataHash} transaction_id 
  * @param {Bip32PrivateKey} secret_key 
  * @returns {Promise<Witness>} 
  */
  static for_legacy_icarus_utxo(genesis_hash: Hash, transaction_id: TransactionSignDataHash, secret_key: Bip32PrivateKey): Promise<Witness>;
}

/**
*/
export class Witnesses extends Ptr {
  /**
  * @returns {Promise<Witnesses>} 
  */
  static new(): Promise<Witnesses>;

  /**
  * @returns {Promise<number>} 
  */
  size(): Promise<number>;

  /**
  * @param {number} index 
  * @returns {Promise<Witness>} 
  */
  get(index: number): Promise<Witness>;

  /**
  * @param {Witness} item 
  * @returns {Promise<void>}
  */
  add(item: Witness): Promise<void>;
}

/**
*/
export class PayloadAuthData extends Ptr {
  /**
  * @returns {Promise<PayloadAuthData>} 
  */
  static for_no_payload(): Promise<PayloadAuthData>;
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
  get_auth_data(): Promise<TransactionBindingAuthData>;

  /**
  * Set the authenticated data
  * @param {PayloadAuthData} auth 
  * @returns {Promise<Transaction>} 
  */
  set_payload_auth(auth: PayloadAuthData): Promise<Transaction>;
}

/**
*/
export class TransactionBuilderSetWitness extends Ptr {
  /**
  * @returns {Promise<TransactionSignDataHash>} 
  */
  get_auth_data_for_witness(): Promise<TransactionSignDataHash>;

  /**
  * @param {Witnesses} witnesses 
  * @returns {Promise<TransactionBuilderSetAuthData>} 
  */
  set_witnesses(witnesses: Witnesses): Promise<TransactionBuilderSetAuthData>;
}

/**
*/
export class TransactionBuilderSetIOs extends Ptr {
  /**
  * @param {Inputs} inputs 
  * @param {Outputs} outputs 
  * @returns {Promise<TransactionBuilderSetWitness>} 
  */
  set_ios(inputs: Inputs, outputs: Outputs): Promise<TransactionBuilderSetWitness>;
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
  static new(): Promise<TransactionBuilder>;

  /**
  * @param {Certificate} cert 
  * @returns {Promise<TransactionBuilderSetIOs>} 
  */
  payload(cert: Certificate): Promise<TransactionBuilderSetIOs>;

  /**
  * @returns {Promise<TransactionBuilderSetIOs>} 
  */
  no_payload(): Promise<TransactionBuilderSetIOs>;
}

/**
*/
export class Payload extends Ptr {
  /**
  * @returns {Promise<Payload>} 
  */
  static no_payload(): Promise<Payload>;
}

/**
*/
export class InputOutput extends Ptr {
  /**
  * @returns {Promise<Inputs>} 
  */
  inputs(): Promise<Inputs>;

  /**
  * @returns {Promise<Outputs>} 
  */
  outputs(): Promise<Outputs>;
}

/**
*/
export class InputOutputBuilder extends Ptr {
  /**
  * @returns {Promise<InputOutputBuilder>} 
  */
  static empty(): Promise<InputOutputBuilder>;

  /**
  * Add input to the IO Builder
  * @param {Input} input 
  * @returns {Promise<void>}
  */
  add_input(input: Input): Promise<void>;

  /**
  * Add output to the IO Builder
  * @param {Address} address 
  * @param {Value} value 
  * @returns {Promise<void>}
  */
  add_output(address: Address, value: Value): Promise<void>;

  /**
  * Estimate fee with the currently added inputs, outputs and certificate based on the given algorithm
  * @param {Fee} fee 
  * @param {Payload} payload 
  * @returns {Promise<Value>} 
  */
  estimate_fee(fee: Fee, payload: Payload): Promise<Value>;

  /**
  * @returns {Promise<InputOutput>} 
  */
  build(): Promise<InputOutput>;

  /**
  * Seal the transaction by passing fee rule and the output policy
  * @param {Payload} payload 
  * @param {Fee} fee_algorithm 
  * @param {OutputPolicy} policy 
  * @returns {Promise<InputOutput>} 
  */
  seal_with_output_policy(payload: Payload, fee_algorithm: Fee, policy: OutputPolicy): Promise<InputOutput>;
}

/**
*/
export class StakeDelegationAuthData extends Ptr {
  /**
  * @param {AccountBindingSignature} signature 
  * @returns {Promise<StakeDelegationAuthData>} 
  */
  static new(signature: AccountBindingSignature): Promise<StakeDelegationAuthData>;
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
  static non_delegated(): Promise<DelegationType>;

  /**
  * @param {PoolId} pool_id 
  * @returns {Promise<DelegationType>} 
  */
  static full(pool_id: PoolId): Promise<DelegationType>;

  /**
  * @param {DelegationRatio} r 
  * @returns {Promise<DelegationType>} 
  */
  static ratio(r: DelegationRatio): Promise<DelegationType>;

  /**
  * @returns {Promise<number>} 
  */
  get_kind(): Promise<number>;

  /**
  * @returns {Promise<Optional<PoolId>>} 
  */
  get_full(): Promise<Optional<PoolId>>;
}

/**
*/
export class AccountIdentifier extends Ptr {}

/**
*/
export class StakeDelegation extends Ptr {
  /**
  * Create a stake delegation object from account (stake key) to pool_id
  * @param {DelegationType} delegation_type 
  * @param {PublicKey} account 
  * @returns {Promise<StakeDelegation>} 
  */
  static new(delegation_type: DelegationType, account: PublicKey): Promise<StakeDelegation>;

  /**
  * @returns {Promise<DelegationType>} 
  */
  delegation_type(): Promise<DelegationType>;

  /**
  * @returns {Promise<AccountIdentifier>} 
  */
  account(): Promise<AccountIdentifier>;

  /**
  * @returns {Promise<Uint8Array>} 
  */
  as_bytes(): Promise<Uint8Array>;

  /**
  * @param {Uint8Array} bytes 
  * @returns {Promise<StakeDelegation>} 
  */
  static from_bytes(bytes: Uint8Array): Promise<StakeDelegation>;
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
  * @param {StakeDelegation} stake_delegation 
  * @returns {Promise<Certificate>} 
  */
  static stake_delegation(stake_delegation: StakeDelegation): Promise<Certificate>;

  /**
  * Create a Certificate for PoolRegistration
  * @param {PoolRegistration} pool_registration 
  * @returns {Promise<Certificate>} 
  */
  static stake_pool_registration(pool_registration: PoolRegistration): Promise<Certificate>;

  /**
  * Create a Certificate for PoolRetirement
  * @param {PoolRetirement} pool_retirement 
  * @returns {Promise<Certificate>} 
  */
  static stake_pool_retirement(pool_retirement: PoolRetirement): Promise<Certificate>;

  /**
  * @returns {Promise<CertificateKind>} 
  */
  get_type(): Promise<CertificateKind>;

  /**
  * @returns {Promise<StakeDelegation>} 
  */
  get_stake_delegation(): Promise<StakeDelegation>;

  /**
  * @returns {Promise<OwnerStakeDelegation>} 
  */
  get_owner_stake_delegation(): Promise<OwnerStakeDelegation>;

  /**
  * @returns {Promise<PoolRegistration>} 
  */
  get_pool_registration(): Promise<PoolRegistration>;

  /**
  * @returns {Promise<PoolRetirement>} 
  */
  get_pool_retirement(): Promise<PoolRetirement>;
}

/**
*/
export class AccountBindingSignature extends Ptr {
  /**
  * @param {PrivateKey} private_key 
  * @param {TransactionBindingAuthData} auth_data 
  * @returns {Promise<AccountBindingSignature>} 
  */
  static new_single(private_key: PrivateKey, auth_data: TransactionBindingAuthData): Promise<AccountBindingSignature>;
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
  derive(index: number): Promise<Bip32PrivateKey>;

  /**
  * @returns {Promise<Bip32PrivateKey>} 
  */
  static generate_ed25519_bip32(): Promise<Bip32PrivateKey>;

  /**
  * @returns {Promise<PrivateKey>} 
  */
  to_raw_key(): Promise<PrivateKey>;

  /**
  * @returns {Promise<Bip32PublicKey>} 
  */
  to_public(): Promise<Bip32PublicKey>;

  /**
  * @param {Uint8Array} bytes 
  * @returns {Promise<Bip32PrivateKey>} 
  */
  static from_bytes(bytes: Uint8Array): Promise<Bip32PrivateKey>;

  /**
  * @returns {Promise<Uint8Array>} 
  */
  as_bytes(): Promise<Uint8Array>;

  /**
  * @param {string} bech32_str 
  * @returns {Promise<Bip32PrivateKey>} 
  */
  static from_bech32(bech32_str: string): Promise<Bip32PrivateKey>;

  /**
  * @returns {Promise<string>} 
  */
  to_bech32(): Promise<string>;

  /**
  * @param {Uint8Array} entropy 
  * @param {Uint8Array} password 
  * @returns {Promise<Bip32PrivateKey>} 
  */
  static from_bip39_entropy(entropy: Uint8Array, password: Uint8Array): Promise<Bip32PrivateKey>;
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
  derive(index: number): Promise<Bip32PublicKey>;

  /**
  * @returns {Promise<PublicKey>} 
  */
  to_raw_key(): Promise<PublicKey>;

  /**
  * @param {Uint8Array} bytes 
  * @returns {Promise<Bip32PublicKey>} 
  */
  static from_bytes(bytes: Uint8Array): Promise<Bip32PublicKey>;

  /**
  * @returns {Promise<Uint8Array>} 
  */
  as_bytes(): Promise<Uint8Array>;

  /**
  * @param {string} bech32_str 
  * @returns {Promise<Bip32PublicKey>} 
  */
  static from_bech32(bech32_str: string): Promise<Bip32PublicKey>;

  /**
  * @returns {Promise<string>} 
  */
  to_bech32(): Promise<string>;
}
