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
    public final void fragmentIdFromBytes(String bytes, Promise promise) {
        Native.I
                .fragmentIdFromBytes(Base64.decode(bytes, Base64.DEFAULT))
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
