package io.emurgo.chainlibs;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;

import android.util.Base64;
import java.util.HashMap;
import java.util.Map;

import androidx.annotation.NonNull;

public class ChainLibsModule extends ReactContextBaseJavaModule {

    private final ReactApplicationContext reactContext;

    public ChainLibsModule(ReactApplicationContext reactContext) {
        super(reactContext);
        this.reactContext = reactContext;
    }

    @NonNull
    public String getName() {
        return "ChainLibs";
    }

    public Map<String, Object> getConstants() {
        HashMap<String, Object> map = new HashMap<>();
        map.put("AddressDiscrimination",  Native.I.AddressDiscrimination());
        map.put("AddressKind",  Native.I.AddressKind());
        map.put("CertificateKind",  Native.I.CertificateKind());
        return map;
    }

    @ReactMethod
    public final void valueFromStr(String str, Promise promise) {
        Native.I
                .valueFromStr(str)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void valueToStr(String valPtr, Promise promise) {
        Native.I
                .valueToStr(new RPtr(valPtr))
                .pour(promise);
    }

    @ReactMethod
    public final void valueCheckedAdd(String valPtr, String otherPtr, Promise promise) {
        Native.I
                .valueCheckedAdd(new RPtr(valPtr), new RPtr(otherPtr))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void valueCheckedSub(String valPtr, String otherPtr, Promise promise) {
        Native.I
                .valueCheckedSub(new RPtr(valPtr), new RPtr(otherPtr))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void publicKeyFromBech32(String bech32, Promise promise) {
        Native.I.publicKeyFromBech32(bech32)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void publicKeyAsBytes(String pubPtr, Promise promise) {
        Native.I
                .publicKeyAsBytes(new RPtr(pubPtr))
                .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
                .pour(promise);
    }

    // Address

    @ReactMethod
    public final void addressFromBytes(String bytes, Promise promise) {
        Native.I
                .addressFromBytes(Base64.decode(bytes, Base64.DEFAULT))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void addressAsBytes(String address, Promise promise) {
        Native.I
                .addressAsBytes(new RPtr(address))
                .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
                .pour(promise);
    }

    @ReactMethod
    public final void addressFromString(String str, Promise promise) {
        Native.I
                .addressFromString(str)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void addressToString(String address, String prefix, Promise promise) {
        Native.I
                .addressToString(new RPtr(address), prefix)
                .pour(promise);
    }

    @ReactMethod
    public final void addressSingleFromPublicKey(String key, Integer discrimination, Promise promise) {
        Native.I
                .addressSingleFromPublicKey(new RPtr(key), discrimination)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void addressDelegationFromPublicKey(String key, String delegation, Integer discrimination, Promise promise) {
        Native.I
                .addressDelegationFromPublicKey(new RPtr(key), new RPtr(delegation), discrimination)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void addressAccountFromPublicKey(String key, Integer discrimination, Promise promise) {
        Native.I
                .addressAccountFromPublicKey(new RPtr(key), discrimination)
                .map(RPtr::toJs)
                .pour(promise);
	}

    @ReactMethod
    public final void addressGetDiscrimination(String address, Promise promise) {
        Native.I
                .addressGetDiscrimination(new RPtr(address))
                .pour(promise);
    }

    @ReactMethod
    public final void addressGetKind(String address, Promise promise) {
        Native.I
                .addressGetKind(new RPtr(address))
                .pour(promise);
    }

    @ReactMethod
    public final void addressToSingleAddress(String address, Promise promise) {
        Native.I
                .addressToSingleAddress(new RPtr(address))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void addressToGroupAddress(String address, Promise promise) {
        Native.I
                .addressToGroupAddress(new RPtr(address))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void addressToAccountAddress(String address, Promise promise) {
        Native.I
                .addressToAccountAddress(new RPtr(address))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Fragment

    @ReactMethod
    public final void fragmentFromTransaction(String tx, Promise promise) {
        Native.I
                .fragmentFromTransaction(new RPtr(tx))
                .map(RPtr::toJs)
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentGetTransaction(String fragment, Promise promise) {
        Native.I
                .fragmentGetTransaction(new RPtr(fragment))
                .map(RPtr::toJs)
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentAsBytes(String fragment, Promise promise) {
        Native.I
                .fragmentAsBytes(new RPtr(fragment))
                .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsInitial(String fragment, Promise promise) {
        Native.I
                .fragmentIsInitial(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsTransaction(String fragment, Promise promise) {
        Native.I
                .fragmentIsTransaction(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsOwnerStakeDelegation(String fragment, Promise promise) {
        Native.I
                .fragmentIsOwnerStakeDelegation(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsStakeDelegation(String fragment, Promise promise) {
        Native.I
                .fragmentIsStakeDelegation(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsPoolRegistration(String fragment, Promise promise) {
        Native.I
                .fragmentIsPoolRegistration(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsPoolRetirement(String fragment, Promise promise) {
        Native.I
                .fragmentIsPoolRetirement(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsPoolUpdate(String fragment, Promise promise) {
        Native.I
                .fragmentIsPoolUpdate(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsOldUtxoDeclaration(String fragment, Promise promise) {
        Native.I
                .fragmentIsOldUtxoDeclaration(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsUpdateProposal(String fragment, Promise promise) {
        Native.I
                .fragmentIsUpdateProposal(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentIsUpdateVote(String fragment, Promise promise) {
        Native.I
                .fragmentIsUpdateVote(new RPtr(fragment))
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentId(String fragment, Promise promise) {
        Native.I
                .fragmentId(new RPtr(fragment))
                .map(RPtr::toJs)
                .pour(promise);
	}

    // TransactionBuilder

    @ReactMethod
    public final void transactionBuilderNew(Promise promise) {
        Native.I
                .transactionBuilderNew()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionBuilderPayload(String txBuilder, String cert, Promise promise) {
        Native.I
                .transactionBuilderPayload(new RPtr(txBuilder), new RPtr(cert))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionBuilderNoPayload(String txBuilder, Promise promise) {
        Native.I
                .transactionBuilderNoPayload(new RPtr(txBuilder))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // TransactionBuilderSetIOs

    @ReactMethod
    public final void transactionBuilderSetIOsSetIOs(String txBuilderSetIOs, String inputs, String outputs, Promise promise) {
        Native.I
                .transactionBuilderSetIOsSetIOs(new RPtr(txBuilderSetIOs), new RPtr(inputs), new RPtr(outputs))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // TransactionBuilderSetWitness

    @ReactMethod
    public final void transactionBuilderSetWitnessGetAuthDataForWitness(String txBuilderSetWitness, Promise promise) {
        Native.I
                .transactionBuilderSetWitnessGetAuthDataForWitness(new RPtr(txBuilderSetWitness))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionBuilderSetWitnessSetWitnesses(String txBuilderSetWitness, String witnesses, Promise promise) {
        Native.I
                .transactionBuilderSetWitnessSetWitnesses(new RPtr(txBuilderSetWitness), new RPtr(witnesses))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // TransactionBuilderSetAuthData

    @ReactMethod
    public final void transactionBuilderSetAuthDataGetAuthData(String txBuilderSetAuthData, Promise promise) {
        Native.I
                .transactionBuilderSetAuthDataGetAuthData(new RPtr(txBuilderSetAuthData))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionBuilderSetAuthDataSetPayloadAuth(String txBuilderSetAuthData, String auth, Promise promise) {
        Native.I
                .transactionBuilderSetAuthDataSetPayloadAuth(new RPtr(txBuilderSetAuthData), new RPtr(auth))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Account

    @ReactMethod
    public final void accountFromAddress(String address, Promise promise) {
        Native.I
                .accountFromAddress(new RPtr(address))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void accountSingleFromPublicKey(String key, Promise promise) {
        Native.I
                .accountSingleFromPublicKey(new RPtr(key))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Input

    @ReactMethod
    public final void inputFromAccount(String account, String v, Promise promise) {
        Native.I
                .inputFromAccount(new RPtr(account), new RPtr(v))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void inputValue(String input, Promise promise) {
        Native.I
                .inputValue(new RPtr(input))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Inputs

    @ReactMethod
    public final void inputsNew(Promise promise) {
        Native.I
                .inputsNew()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void inputsSize(String inputs, Promise promise) {
        Native.I
                .inputsSize(new RPtr(inputs))
                .map(Long::intValue)
                .pour(promise);
    }

    @ReactMethod
    public final void inputsGet(String inputs, Integer index, Promise promise) {
        Native.I
                .inputsGet(new RPtr(inputs), index)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void inputsAdd(String inputs, String item, Promise promise) {
        Native.I
                .inputsAdd(new RPtr(inputs), new RPtr(item))
                .pour(promise);
    }

    // Fee

    @ReactMethod
    public final void feeLinearFee(String constant, String coefficient, String certificate, Promise promise) {
        Native.I
                .feeLinearFee(new RPtr(constant), new RPtr(coefficient), new RPtr(certificate))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void feeCalculate(String fee, String tx, Promise promise) {
        Native.I
                .feeCalculate(new RPtr(fee), new RPtr(tx))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // OutputPolicy

    @ReactMethod
    public final void outputPolicyForget(Promise promise) {
        Native.I
                .outputPolicyForget()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void outputPolicyOne(String address, Promise promise) {
        Native.I
                .outputPolicyOne(new RPtr(address))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Witness

    @ReactMethod
    public final void witnessForAccount(String genesisHash, String transactionId, String secretKey, String accountSpendingCounter, Promise promise) {
        Native.I
                .witnessForAccount(new RPtr(genesisHash), new RPtr(transactionId), new RPtr(secretKey), new RPtr(accountSpendingCounter))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void witnessForUtxo(String genesisHash, String transactionId, String secretKey, Promise promise) {
        Native.I
                .witnessForUtxo(new RPtr(genesisHash), new RPtr(transactionId), new RPtr(secretKey))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void witnessForLegacyIcarusUtxo(String genesisHash, String transactionId, String secretKey, Promise promise) {
        Native.I
                .witnessForLegacyIcarusUtxo(new RPtr(genesisHash), new RPtr(transactionId), new RPtr(secretKey))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // PrivateKey

    @ReactMethod
    public final void privateKeyFromBech32(String bech32Str, Promise promise) {
        Native.I
                .privateKeyFromBech32(bech32Str)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void privateKeyToPublic(String prvPtr, Promise promise) {
        Native.I
                .privateKeyToPublic(new RPtr(prvPtr))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void privateKeyFromExtendedBytes(String bytes, Promise promise) {
        Native.I
                .privateKeyFromExtendedBytes(Base64.decode(bytes, Base64.DEFAULT))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Hash

    @ReactMethod
    public final void hashFromHex(String hexString, Promise promise) {
        Native.I
                .hashFromHex(hexString)
                .map(RPtr::toJs)
                .pour(promise);
    }

    // SpendingCounter

    @ReactMethod
    public final void spendingCounterZero(Promise promise) {
        Native.I
                .spendingCounterZero()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void spendingCounterFromU32(Integer counter, Promise promise) {
        Native.I
                .spendingCounterFromU32(counter)
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Transaction

    @ReactMethod
    public final void transactionId(String transaction, Promise promise) {
        Native.I
                .transactionId(new RPtr(transaction))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionInputs(String transaction, Promise promise) {
        Native.I
                .transactionInputs(new RPtr(transaction))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionOutputs(String transaction, Promise promise) {
        Native.I
                .transactionOutputs(new RPtr(transaction))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Output

    @ReactMethod
    public final void outputAddress(String output, Promise promise) {
        Native.I
                .outputAddress(new RPtr(output))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void outputValue(String output, Promise promise) {
        Native.I
                .outputAddress(new RPtr(output))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Outputs

    @ReactMethod
    public final void outputsNew(Promise promise) {
        Native.I
                .outputsNew()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void outputsSize(String outputs, Promise promise) {
        Native.I
                .outputsSize(new RPtr(outputs))
                .map(Long::intValue)
                .pour(promise);
    }

    @ReactMethod
    public final void outputsGet(String outputs, Integer index, Promise promise) {
        Native.I
                .outputsGet(new RPtr(outputs), index)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void outputsAdd(String outputs, String item, Promise promise) {
        Native.I
                .outputsAdd(new RPtr(outputs), new RPtr(item))
                .pour(promise);
    }

    // FragmentId

    @ReactMethod
    public final void fragmentIdCalculate(String bytes, Promise promise) {
        Native.I
                .fragmentIdCalculate(Base64.decode(bytes, Base64.DEFAULT))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void fragmentIdAsBytes(String fragmentId, Promise promise) {
        Native.I
                .fragmentIdAsBytes(new RPtr(fragmentId))
                .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
                .pour(promise);
    }

    // TransactionSignDataHash

    @ReactMethod
    public final void transactionSignDataHashFromBytes(String bytes, Promise promise) {
        Native.I
                .transactionSignDataHashFromBytes(Base64.decode(bytes, Base64.DEFAULT))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionSignDataHashFromHex(String input, Promise promise) {
        Native.I
                .transactionSignDataHashFromHex(input)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionSignDataHashAsBytes(String txSignDataHash, Promise promise) {
        Native.I
                .transactionSignDataHashAsBytes(new RPtr(txSignDataHash))
                .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
                .pour(promise);
    }

    // UtxoPointer

    @ReactMethod
    public final void utxoPointerNew(String fragmentId, Integer outputIndex, String value, Promise promise) {
        Native.I
                .utxoPointerNew(new RPtr(fragmentId), outputIndex, new RPtr(value))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Balance

    @ReactMethod
    public final void balanceIsPositive(String balance, Promise promise) {
        Native.I
                .balanceIsPositive(new RPtr(balance))
                .pour(promise);
    }

    @ReactMethod
    public final void balanceIsNegative(String balance, Promise promise) {
        Native.I
                .balanceIsNegative(new RPtr(balance))
                .pour(promise);
    }

    @ReactMethod
    public final void balanceIsZero(String balance, Promise promise) {
        Native.I
                .balanceIsZero(new RPtr(balance))
                .pour(promise);
    }

    @ReactMethod
    public final void balanceGetValue(String balance, Promise promise) {
        Native.I
                .balanceGetValue(new RPtr(balance))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // InputOutputBuilder

    @ReactMethod
    public final void inputOutputBuilderEmpty(Promise promise) {
        Native.I
                .inputOutputBuilderEmpty()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void inputOutputBuilderAddInput(String ioBuilder, String input, Promise promise) {
        Native.I
                .inputOutputBuilderAddInput(new RPtr(ioBuilder), new RPtr(input))
                .pour(promise);
    }

    @ReactMethod
    public final void inputOutputBuilderAddOutput(String ioBuilder, String address, String value, Promise promise) {
        Native.I
                .inputOutputBuilderAddOutput(new RPtr(ioBuilder), new RPtr(address), new RPtr(value))
                .pour(promise);
    }

    @ReactMethod
    public final void inputOutputBuilderEstimateFee(String ioBuilder, String fee, String payload, Promise promise) {
        Native.I
                .inputOutputBuilderEstimateFee(new RPtr(ioBuilder), new RPtr(fee), new RPtr(payload))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void inputOutputBuilderBuild(String ioBuilder, Promise promise) {
        Native.I
                .inputOutputBuilderBuild(new RPtr(ioBuilder))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void inputOutputBuilderSealWithOutputPolicy(String ioBuilder, String payload, String feeAlgorithm, String policy, Promise promise) {
        Native.I
                .inputOutputBuilderSealWithOutputPolicy(new RPtr(ioBuilder), new RPtr(payload), new RPtr(feeAlgorithm), new RPtr(policy))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // InputOutput

    @ReactMethod
    public final void inputOutputInputs(String inputOutput, Promise promise) {
        Native.I
                .inputOutputInputs(new RPtr(inputOutput))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void inputOutputOutputs(String inputOutput, Promise promise) {
        Native.I
                .inputOutputOutputs(new RPtr(inputOutput))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Witnesses

    @ReactMethod
    public final void witnessesNew(Promise promise) {
        Native.I
                .witnessesNew()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void witnessesSize(String witnesses, Promise promise) {
        Native.I
                .witnessesSize(new RPtr(witnesses))
                .map(Long::intValue)
                .pour(promise);
    }

    @ReactMethod
    public final void witnessesGet(String witnesses, Integer index, Promise promise) {
        Native.I
                .witnessesGet(new RPtr(witnesses), index)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void witnessesAdd(String witnesses, String item, Promise promise) {
        Native.I
                .witnessesAdd(new RPtr(witnesses), new RPtr(item))
                .pour(promise);
    }

    // PayloadAuthData

    @ReactMethod
    public final void payloadAuthDataForNoPayload(Promise promise) {
        Native.I
                .payloadAuthDataForNoPayload()
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Payload

    @ReactMethod
    public final void payloadNoPayload(Promise promise) {
        Native.I
                .payloadNoPayload()
                .map(RPtr::toJs)
                .pour(promise);
    }

    // StakeDelegationAuthData

    @ReactMethod
    public final void stakeDelegationAuthDataNew(String signature, Promise promise) {
        Native.I
                .stakeDelegationAuthDataNew(new RPtr(signature))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // StakeDelegation

    @ReactMethod
    public final void stakeDelegationNew(String delegationType, String account, Promise promise) {
        Native.I
                .stakeDelegationNew(new RPtr(delegationType), new RPtr(account))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void stakeDelegationDelegationType(String stakeDelegation, Promise promise) {
        Native.I
                .stakeDelegationDelegationType(new RPtr(stakeDelegation))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void stakeDelegationAccount(String stakeDelegation, Promise promise) {
        Native.I
                .stakeDelegationAccount(new RPtr(stakeDelegation))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void stakeDelegationAsBytes(String stakeDelegation, Promise promise) {
        Native.I
                .stakeDelegationAsBytes(new RPtr(stakeDelegation))
                .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
                .pour(promise);
    }

    @ReactMethod
    public final void stakeDelegationFromBytes(String bytes, Promise promise) {
        Native.I
                .stakeDelegationFromBytes(Base64.decode(bytes, Base64.DEFAULT))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Certificate

    @ReactMethod
    public final void certificateStakeDelegation(String stakeDelegation, Promise promise) {
        Native.I
                .certificateStakeDelegation(new RPtr(stakeDelegation))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void certificateStakePoolRegistration(String poolRegistration, Promise promise) {
        Native.I
                .certificateStakePoolRegistration(new RPtr(poolRegistration))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void certificateStakePoolRetirement(String poolRetirement, Promise promise) {
        Native.I
                .certificateStakePoolRetirement(new RPtr(poolRetirement))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void certificateGetType(String certificate, Promise promise) {
        Native.I
                .certificateGetType(new RPtr(certificate))
                .pour(promise);
    }

    @ReactMethod
    public final void certificateGetStakeDelegation(String certificate, Promise promise) {
        Native.I
                .certificateGetStakeDelegation(new RPtr(certificate))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void certificateGetOwnerStakeDelegation(String certificate, Promise promise) {
        Native.I
                .certificateGetOwnerStakeDelegation(new RPtr(certificate))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void certificateGetPoolRegistration(String certificate, Promise promise) {
        Native.I
                .certificateGetPoolRegistration(new RPtr(certificate))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void certificateGetPoolRetirement(String certificate, Promise promise) {
        Native.I
                .certificateGetPoolRetirement(new RPtr(certificate))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // AccountBindingSignature

    @ReactMethod
    public final void accountBindingSignatureNewSingle(String privateKey, String authData, Promise promise) {
        Native.I
                .accountBindingSignatureNewSingle(new RPtr(privateKey), new RPtr(authData))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Bip32PrivateKey

    @ReactMethod
    public final void bip32PrivateKeyDerive(String bip32PrivateKey, Integer index, Promise promise) {
        Native.I
                .bip32PrivateKeyDerive(new RPtr(bip32PrivateKey), index)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PrivateKeyGenerateEd25519Bip32(Promise promise) {
        Native.I
                .bip32PrivateKeyGenerateEd25519Bip32()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PrivateKeyToRawKey(String bip32PrivateKey, Promise promise) {
        Native.I
                .bip32PrivateKeyToRawKey(new RPtr(bip32PrivateKey))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PrivateKeyToPublic(String bip32PrivateKey, Promise promise) {
        Native.I
                .bip32PrivateKeyToPublic(new RPtr(bip32PrivateKey))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PrivateKeyFromBytes(String bytes, Promise promise) {
        Native.I
                .bip32PrivateKeyFromBytes(Base64.decode(bytes, Base64.DEFAULT))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PrivateKeyAsBytes(String bip32PrivateKey, Promise promise) {
        Native.I
                .bip32PrivateKeyAsBytes(new RPtr(bip32PrivateKey))
                .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PrivateKeyFromBech32(String bech32Str, Promise promise) {
        Native.I
                .bip32PrivateKeyFromBech32(bech32Str)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PrivateKeyToBech32(String bip32PrivateKey, Promise promise) {
        Native.I
                .bip32PrivateKeyToBech32(new RPtr(bip32PrivateKey))
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PrivateKeyFromBip39Entropy(String entropy, String password, Promise promise) {
        Native.I
                .bip32PrivateKeyFromBip39Entropy(Base64.decode(entropy, Base64.DEFAULT), Base64.decode(password, Base64.DEFAULT))
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Bip32PublicKey

    @ReactMethod
    public final void bip32PublicKeyDerive(String bip32PublicKey, Integer index, Promise promise) {
        Native.I
                .bip32PublicKeyDerive(new RPtr(bip32PublicKey), index)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PublicKeyToRawKey(String bip32PublicKey, Promise promise) {
        Native.I
                .bip32PublicKeyToRawKey(new RPtr(bip32PublicKey))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PublicKeyFromBytes(String bytes, Promise promise) {
        Native.I
                .bip32PublicKeyFromBytes(Base64.decode(bytes, Base64.DEFAULT))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PublicKeyAsBytes(String bip32PublicKey, Promise promise) {
        Native.I
                .bip32PublicKeyAsBytes(new RPtr(bip32PublicKey))
                .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PublicKeyFromBech32(String bech32Str, Promise promise) {
        Native.I
                .bip32PublicKeyFromBech32(bech32Str)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void bip32PublicKeyToBech32(String bip32PublicKey, Promise promise) {
        Native.I
                .bip32PublicKeyToBech32(new RPtr(bip32PublicKey))
                .pour(promise);
    }

    // DelegationType

    @ReactMethod
    public final void delegationTypeNonDelegated(Promise promise) {
        Native.I
                .delegationTypeNonDelegated()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void delegationTypeFull(String poolId, Promise promise) {
        Native.I
                .delegationTypeFull(new RPtr(poolId))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void delegationTypeRatio(String r, Promise promise) {
        Native.I
                .delegationTypeRatio(new RPtr(r))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void delegationTypeGetKind(String delegationType, Promise promise) {
        Native.I
                .delegationTypeGetKind(new RPtr(delegationType))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void delegationTypeGetFull(String delegationType, Promise promise) {
        Native.I
                .delegationTypeGetFull(new RPtr(delegationType))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void ptrFree(String ptr, Promise promise) {
        try {
            (new RPtr(ptr)).free();
            promise.resolve(null);
        } catch (Throwable err) {
            promise.reject(err);
        }
    }
}
