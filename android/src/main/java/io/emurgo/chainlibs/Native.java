package io.emurgo.chainlibs;

import java.util.Map;

final class Native {
    static final Native I;

    static {
        I = new Native();
        System.loadLibrary("react_native_chain_libs");
        I.initLibrary();
    }

    private Native() { }

    private native void initLibrary();

    public final native Map<String, Object> AddressDiscrimination();
    public final native Map<String, Object> AddressKind();
    public final native Map<String, Object> CertificateKind();

    // Value
    public final native Result<RPtr> valueFromStr(String str);
    public final native Result<RPtr> valueFromU64(long u64);
    public final native Result<String> valueToStr(RPtr value);
    public final native Result<RPtr> valueCheckedAdd(RPtr value, RPtr other);
    public final native Result<RPtr> valueCheckedSub(RPtr value, RPtr other);

    // PublicKey
    public final native Result<RPtr> publicKeyFromBech32(String bech32);
    public final native Result<byte[]> publicKeyAsBytes(RPtr pubKey);

    // Address
    public final native Result<RPtr> addressFromBytes(byte[] bytes);
    public final native Result<byte[]> addressAsBytes(RPtr address);
    public final native Result<RPtr> addressFromString(String str);
    public final native Result<String> addressToString(RPtr address, String prefix);
    public final native Result<RPtr> addressSingleFromPublicKey(RPtr key, int discrimination);
    public final native Result<RPtr> addressDelegationFromPublicKey(RPtr key, RPtr delegation, int discrimination);
    public final native Result<RPtr> addressAccountFromPublicKey(RPtr key, int discrimination);
    public final native Result<Integer> addressGetDiscrimination(RPtr address);
    public final native Result<Integer> addressGetKind(RPtr address);
    public final native Result<RPtr> addressToSingleAddress(RPtr address);
    public final native Result<RPtr> addressToGroupAddress(RPtr address);
    public final native Result<RPtr> addressToAccountAddress(RPtr address);

    // Fragment
    public final native Result<RPtr> fragmentFromTransaction(RPtr tx);
    public final native Result<RPtr> fragmentGetTransaction(RPtr fragment);
    public final native Result<byte[]> fragmentAsBytes(RPtr fragment);
    public final native Result<Boolean> fragmentIsInitial(RPtr fragment);
    public final native Result<Boolean> fragmentIsTransaction(RPtr fragment);
    public final native Result<Boolean> fragmentIsOwnerStakeDelegation(RPtr fragment);
    public final native Result<Boolean> fragmentIsStakeDelegation(RPtr fragment);
    public final native Result<Boolean> fragmentIsPoolRegistration(RPtr fragment);
    public final native Result<Boolean> fragmentIsPoolRetirement(RPtr fragment);
    public final native Result<Boolean> fragmentIsPoolUpdate(RPtr fragment);
    public final native Result<Boolean> fragmentIsOldUtxoDeclaration(RPtr fragment);
    public final native Result<Boolean> fragmentIsUpdateProposal(RPtr fragment);
    public final native Result<Boolean> fragmentIsUpdateVote(RPtr fragment);
    public final native Result<RPtr> fragmentId(RPtr fragment);

    // TransactionBuilder
    public final native Result<RPtr> transactionBuilderNew();
    public final native Result<RPtr> transactionBuilderPayload(RPtr txBuilder, RPtr cert);
    public final native Result<RPtr> transactionBuilderNoPayload(RPtr txBuilder);

    // TransactionBuilderSetIOs
    public final native Result<RPtr> transactionBuilderSetIOsSetIOs(RPtr txBuilderSetIOs, RPtr inputs, RPtr outputs);

    // TransactionBuilderSetWitness
    public final native Result<RPtr> transactionBuilderSetWitnessGetAuthDataForWitness(RPtr txBuilderSetWitness);
    public final native Result<RPtr> transactionBuilderSetWitnessSetWitnesses(RPtr txBuilderSetWitness, RPtr witnesses);

    // TransactionBuilderSetAuthData
    public final native Result<RPtr> transactionBuilderSetAuthDataGetAuthData(RPtr txBuilderSetAuthData);
    public final native Result<RPtr> transactionBuilderSetAuthDataSetPayloadAuth(RPtr txBuilderSetAuthData, RPtr auth);

    // Account
    public final native Result<RPtr> accountFromAddress(RPtr address);
    public final native Result<RPtr> accountSingleFromPublicKey(RPtr key);

    // Input
    public final native Result<RPtr> inputFromAccount(RPtr account, RPtr v);
    public final native Result<RPtr> inputValue(RPtr input);

    // Inputs
    public final native Result<RPtr> inputsNew();
    public final native Result<Long> inputsSize(RPtr inputs);
    public final native Result<RPtr> inputsGet(RPtr inputs, long index);
    public final native Result<Void> inputsAdd(RPtr inputs, RPtr item);

    // Fee
    public final native Result<RPtr> feeLinearFee(RPtr constant, RPtr coefficient, RPtr certificate);
    public final native Result<RPtr> feeCalculate(RPtr fee, RPtr tx);

    // OutputPolicy
    public final native Result<RPtr> outputPolicyForget();
    public final native Result<RPtr> outputPolicyOne(RPtr address);

    // Witness
    public final native Result<RPtr> witnessForAccount(RPtr genesisHash, RPtr transactionId, RPtr secretKey, RPtr accountSpendingCounter);
    public final native Result<RPtr> witnessForUtxo(RPtr genesisHash, RPtr transactionId, RPtr secretKey);
    public final native Result<RPtr> witnessForLegacyIcarusUtxo(RPtr genesisHash, RPtr transactionId, RPtr secretKey);

    // PrivateKey
    public final native Result<RPtr> privateKeyFromBech32(String bech32Str);
    public final native Result<RPtr> privateKeyToPublic(RPtr privateKey);
    public final native Result<RPtr> privateKeyFromExtendedBytes(byte[] bytes);

    // Hash
    public final native Result<RPtr> hashFromHex(String hexString);

    // SpendingCounter
    public final native Result<RPtr> spendingCounterZero();
    public final native Result<RPtr> spendingCounterFromU32(int counter);

    // Transaction
    public final native Result<RPtr> transactionId(RPtr transaction);
    public final native Result<RPtr> transactionInputs(RPtr transaction);
    public final native Result<RPtr> transactionOutputs(RPtr transaction);

    // Output
    public final native Result<RPtr> outputAddress(RPtr output);
    public final native Result<RPtr> outputValue(RPtr output);

    // Outputs
    public final native Result<RPtr> outputsNew();
    public final native Result<Long> outputsSize(RPtr outputs);
    public final native Result<RPtr> outputsGet(RPtr outputs, long index);
    public final native Result<Void> outputsAdd(RPtr outputs, RPtr item);

    // FragmentId
    public final native Result<RPtr> fragmentIdCalculate(byte[] bytes);
    public final native Result<byte[]> fragmentIdAsBytes(RPtr fragmentId);

    // TransactionSignDataHash
    public final native Result<RPtr> transactionSignDataHashFromBytes(byte[] bytes);
    public final native Result<RPtr> transactionSignDataHashFromHex(String input);
    public final native Result<byte[]> transactionSignDataHashAsBytes(RPtr txSignDataHash);

    // UtxoPointer
    public final native Result<RPtr> utxoPointerNew(RPtr fragmentId, int outputIndex, RPtr value);

    // Balance
    public final native Result<Boolean> balanceIsPositive(RPtr balance);
    public final native Result<Boolean> balanceIsNegative(RPtr balance);
    public final native Result<Boolean> balanceIsZero(RPtr balance);
    public final native Result<RPtr> balanceGetValue(RPtr balance);

    // InputOutputBuilder
    public final native Result<RPtr> inputOutputBuilderEmpty();
    public final native Result<Void> inputOutputBuilderAddInput(RPtr ioBuilder, RPtr input);
    public final native Result<Void> inputOutputBuilderAddOutput(RPtr ioBuilder, RPtr address, RPtr value);
    public final native Result<RPtr> inputOutputBuilderEstimateFee(RPtr ioBuilder, RPtr fee, RPtr payload);
    public final native Result<RPtr> inputOutputBuilderBuild(RPtr ioBuilder);
    public final native Result<RPtr> inputOutputBuilderSealWithOutputPolicy(RPtr ioBuilder, RPtr payload, RPtr feeAlgorithm, RPtr policy);

    // InputOutput
    public final native Result<RPtr> inputOutputInputs(RPtr inputOutput);
    public final native Result<RPtr> inputOutputOutputs(RPtr inputOutput);

    // Witnesses
    public final native Result<RPtr> witnessesNew();
    public final native Result<Long> witnessesSize(RPtr witnesses);
    public final native Result<RPtr> witnessesGet(RPtr witnesses, long index);
    public final native Result<Void> witnessesAdd(RPtr witnesses, RPtr item);

    // PayloadAuthData
    public final native Result<RPtr> payloadAuthDataForNoPayload();

    // Payload
    public final native Result<RPtr> payloadNoPayload();

    // StakeDelegationAuthData
    public final native Result<RPtr> stakeDelegationAuthDataNew(RPtr signature);

    // StakeDelegation
    public final native Result<RPtr> stakeDelegationNew(RPtr delegationType, RPtr account);
    public final native Result<RPtr> stakeDelegationDelegationType(RPtr stakeDelegation);
    public final native Result<RPtr> stakeDelegationAccount(RPtr stakeDelegation);
    public final native Result<byte[]> stakeDelegationAsBytes(RPtr stakeDelegation);
    public final native Result<RPtr> stakeDelegationFromBytes(byte[] bytes);

    // Certificate
    public final native Result<RPtr> certificateStakeDelegation(RPtr stakeDelegation);
    public final native Result<RPtr> certificateStakePoolRegistration(RPtr poolRegistration);
    public final native Result<RPtr> certificateStakePoolRetirement(RPtr poolRetirement);
    public final native Result<Integer> certificateGetType(RPtr certificate);
    public final native Result<RPtr> certificateGetStakeDelegation(RPtr certificate);
    public final native Result<RPtr> certificateGetOwnerStakeDelegation(RPtr certificate);
    public final native Result<RPtr> certificateGetPoolRegistration(RPtr certificate);
    public final native Result<RPtr> certificateGetPoolRetirement(RPtr certificate);

    // AccountBindingSignature
    public final native Result<RPtr> accountBindingSignatureNewSingle(RPtr privateKey, RPtr authData);

    // Bip32PrivateKey
    public final native Result<RPtr> bip32PrivateKeyDerive(RPtr bip32PrivateKey, int index);
    public final native Result<RPtr> bip32PrivateKeyGenerateEd25519Bip32();
    public final native Result<RPtr> bip32PrivateKeyToRawKey(RPtr bip32PrivateKey);
    public final native Result<RPtr> bip32PrivateKeyToPublic(RPtr bip32PrivateKey);
    public final native Result<RPtr> bip32PrivateKeyFromBytes(byte[] bytes);
    public final native Result<byte[]> bip32PrivateKeyAsBytes(RPtr bip32PrivateKey);
    public final native Result<RPtr> bip32PrivateKeyFromBech32(String bech32Str);
    public final native Result<String> bip32PrivateKeyToBech32(RPtr bip32PrivateKey);
    public final native Result<RPtr> bip32PrivateKeyFromBip39Entropy(byte[] entropy, byte[] password);

    // Bip32PublicKey
    public final native Result<RPtr> bip32PublicKeyDerive(RPtr bip32PublicKey, int index);
    public final native Result<RPtr> bip32PublicKeyToRawKey(RPtr bip32PublicKey);
    public final native Result<RPtr> bip32PublicKeyFromBytes(byte[] bytes);
    public final native Result<byte[]> bip32PublicKeyAsBytes(RPtr bip32PublicKey);
    public final native Result<RPtr> bip32PublicKeyFromBech32(String bech32Str);
    public final native Result<String> bip32PublicKeyToBech32(RPtr bip32PublicKey);

    // DelegationType
    public final native Result<RPtr> delegationTypeNonDelegated();
    public final native Result<RPtr> delegationTypeFull(RPtr poolId);
    public final native Result<RPtr> delegationTypeRatio(RPtr r);
    public final native Result<RPtr> delegationTypeGetKind(RPtr delegationType);
    public final native Result<RPtr> delegationTypeGetFull(RPtr delegationType);

    public final native void ptrFree(RPtr ptr);
}
