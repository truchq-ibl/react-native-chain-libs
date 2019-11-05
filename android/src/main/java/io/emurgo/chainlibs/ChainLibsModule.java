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

    // AuthenticatedTransaction

    @ReactMethod
    public final void authenticatedTransactionTransaction(String authTx, Promise promise) {
	    Native.I
                .authenticatedTransactionTransaction(new RPtr(authTx))
                .map(RPtr::toJs)
                .pour(promise);
	}

    // Fragment

    @ReactMethod
    public final void fragmentFromAuthenticatedTransaction(String authTx, Promise promise) {
        Native.I
                .fragmentFromAuthenticatedTransaction(new RPtr(authTx))
                .map(RPtr::toJs)
                .pour(promise);
	}

    @ReactMethod
    public final void fragmentFromGeneratedTransaction(String authTx, Promise promise) {
        Native.I
                .fragmentFromGeneratedTransaction(new RPtr(authTx))
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
    public final void fragmentIsPoolManagement(String fragment, Promise promise) {
        Native.I
                .fragmentIsPoolManagement(new RPtr(fragment))
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
    public final void transactionBuilderNewNoPayload(Promise promise) {
        Native.I
                .transactionBuilderNewNoPayload()
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionBuilderAddInput(String txBuilder, String input, Promise promise) {
        Native.I
                .transactionBuilderAddInput(new RPtr(txBuilder), new RPtr(input))
                .pour(promise);
    }

    @ReactMethod
    public final void transactionBuilderAddOutput(String txBuilder, String address, String value, Promise promise) {
        Native.I
                .transactionBuilderAddOutput(new RPtr(txBuilder), new RPtr(address), new RPtr(value))
                .pour(promise);
    }

    @ReactMethod
    public final void transactionBuilderSealWithOutputPolicy(String txBuilder, String fee, String outputPolicy, Promise promise) {
        Native.I
                .transactionBuilderSealWithOutputPolicy(new RPtr(txBuilder), new RPtr(fee), new RPtr(outputPolicy))
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

    // Inputs

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

    // TransactionFinalizer

    @ReactMethod
    public final void transactionFinalizerNew(String transaction, Promise promise) {
        Native.I
                .transactionFinalizerNew(new RPtr(transaction))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionFinalizerGetTxSignDataHash(String txFinalizer, Promise promise) {
        Native.I
                .transactionFinalizerGetTxSignDataHash(new RPtr(txFinalizer))
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionFinalizerSetWitness(String txFinalizer, Integer index, String witness, Promise promise) {
        Native.I
                .transactionFinalizerSetWitness(new RPtr(txFinalizer), index, new RPtr(witness))
                .pour(promise);
    }

    @ReactMethod
    public final void transactionFinalizerBuild(String txFinalizer, Promise promise) {
        Native.I
                .transactionFinalizerBuild(new RPtr(txFinalizer))
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
