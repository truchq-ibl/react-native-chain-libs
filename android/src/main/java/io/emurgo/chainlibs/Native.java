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
    public final native Result<RPtr> addressFromString(String str);
    public final native Result<String> addressToString(RPtr address, String prefix);
    public final native Result<RPtr> addressSingleFromPublicKey(RPtr key, int discrimination);
    public final native Result<RPtr> addressDelegationFromPublicKey(RPtr key, RPtr delegation, int discrimination);
    public final native Result<RPtr> addressAccountFromPublicKey(RPtr key, int discrimination);

    // AuthenticatedTransaction
    public final native Result<RPtr> authenticatedTransactionTransaction(RPtr authTx);

    // Fragment
    public final native Result<RPtr> fragmentFromAuthenticatedTransaction(RPtr authTx);
    public final native Result<RPtr> fragmentFromGeneratedTransaction(RPtr authTx);
    public final native Result<RPtr> fragmentGetTransaction(RPtr fragment);
    public final native Result<byte[]> fragmentAsBytes(RPtr fragment);
    public final native Result<Boolean> fragmentIsInitial(RPtr fragment);
    public final native Result<Boolean> fragmentIsTransaction(RPtr fragment);
    public final native Result<Boolean> fragmentIsOwnerStakeDelegation(RPtr fragment);
    public final native Result<Boolean> fragmentIsStakeDelegation(RPtr fragment);
    public final native Result<Boolean> fragmentIsPoolRegistration(RPtr fragment);
    public final native Result<Boolean> fragmentIsPoolManagement(RPtr fragment);
    public final native Result<Boolean> fragmentIsOldUtxoDeclaration(RPtr fragment);
    public final native Result<Boolean> fragmentIsUpdateProposal(RPtr fragment);
    public final native Result<Boolean> fragmentIsUpdateVote(RPtr fragment);
    public final native Result<RPtr> fragmentId(RPtr fragment);

    // TransactionBuilder
    public final native Result<RPtr> transactionBuilderNewNoPayload();
    public final native Result<Void> transactionBuilderAddInput(RPtr txBuilder, RPtr input);
    public final native Result<Void> transactionBuilderAddOutput(RPtr txBuilder, RPtr address, RPtr value);
    public final native Result<RPtr> transactionBuilderSealWithOutputPolicy(RPtr txBuilder, RPtr fee, RPtr outputPolicy);

    // Account
    public final native Result<RPtr> accountFromAddress(RPtr address);

    // Input
    public final native Result<RPtr> inputFromAccount(RPtr account, RPtr v);

    // Inputs
    public final native Result<Long> inputsSize(RPtr inputs);
    public final native Result<RPtr> inputsGet(RPtr inputs, long index);

    // Fee
    public final native Result<RPtr> feeLinearFee(RPtr constant, RPtr coefficient, RPtr certificate);
    public final native Result<RPtr> feeCalculate(RPtr fee, RPtr tx);

    // OutputPolicy
    public final native Result<RPtr> outputPolicyForget();
    public final native Result<RPtr> outputPolicyOne(RPtr address);

    // TransactionFinalizer
    public final native Result<RPtr> transactionFinalizerNew(RPtr transaction);
    public final native Result<RPtr> transactionFinalizerGetTxSignDataHash(RPtr txFinalizer);
    public final native Result<Void> transactionFinalizerSetWitness(RPtr txFinalizer, long index, RPtr witness);
    public final native Result<RPtr> transactionFinalizerBuild(RPtr txFinalizer);

    // Witness
    public final native Result<RPtr> witnessForAccount(RPtr genesisHash, RPtr transactionId, RPtr secretKey, RPtr accountSpendingCounter);

    // PrivateKey
    public final native Result<RPtr> privateKeyFromBech32(String bech32Str);
    public final native Result<RPtr> privateKeyToPublic();

    // Hash
    public final native Result<RPtr> hashFromHex(String hexString);

    // SpendingCounter
    public final native Result<RPtr> spendingCounterZero();

    // Transaction
    public final native Result<RPtr> transactionId(RPtr transaction);
    public final native Result<RPtr> transactionInputs(RPtr transaction);
    public final native Result<RPtr> transactionOutputs(RPtr transaction);

    // Output
    public final native Result<RPtr> outputAddress(RPtr output);
    public final native Result<RPtr> outputValue(RPtr output);

    // Outputs
    public final native Result<Long> outputsSize(RPtr outputs);
    public final native Result<RPtr> outputsGet(RPtr outputs, long index);

    // FragmentId
    public final native Result<RPtr> fragmentIdFromBytes(byte[] bytes);
    public final native Result<byte[]> fragmentIdAsBytes(RPtr fragmentId);

    // TransactionSignDataHash
    public final native Result<RPtr> transactionSignDataHashFromBytes(byte[] bytes);
    public final native Result<RPtr> transactionSignDataHashFromHex(String input);
    public final native Result<byte[]> transactionSignDataHashAsBytes(RPtr txSignDataHash);

    public final native void ptrFree(RPtr ptr);
}
