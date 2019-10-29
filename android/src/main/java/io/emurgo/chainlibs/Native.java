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
    public final native Result<String> addressToString(RPtr address);
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

    // Account
    public final native Result<RPtr> accountFromAddress(RPtr address);

    // Input
    public final native Result<RPtr> inputFromAccount(RPtr account);

    // Fee
    public final native Result<RPtr> feeLinearFee(RPtr constant, RPtr coefficient, RPtr certificate);

    public final native void ptrFree(RPtr ptr);
}
