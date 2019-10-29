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
    public final void addressToString(String address, Promise promise) {
        Native.I
                .addressToString(new RPtr(address))
                .pour(promise);
    }

    @ReactMethod
    public final void addressSingleFromPublicKey(String key, int discrimination, Promise promise) {
        Native.I
                .addressSingleFromPublicKey(new RPtr(key), discrimination)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void addressDelegationFromPublicKey(String key, String delegation, int discrimination, Promise promise) {
        Native.I
                .addressDelegationFromPublicKey(new RPtr(key), new RPtr(delegation), discrimination)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void addressAccountFromPublicKey(String key, int discrimination, Promise promise) {
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
    public final void transactionBuilderAddInput(RPtr txBuilder, RPtr input, Promise promise) {
        Native.I
                .transactionBuilderAddInput(txBuilder, input)
                .pour(promise);
    }

    @ReactMethod
    public final void transactionBuilderAddOutput(RPtr txBuilder, RPtr address, RPtr value, Promise promise) {
        Native.I
                .transactionBuilderAddOutput(txBuilder, address, value)
                .pour(promise);
    }

    // Account

    @ReactMethod
    public final void accountFromAddress(RPtr address, Promise promise) {
        Native.I
                .accountFromAddress(address)
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Input

    @ReactMethod
    public final void inputFromAccount(RPtr account, Promise promise) {
        Native.I
                .inputFromAccount(account)
                .map(RPtr::toJs)
                .pour(promise);
    }

    // Fee

    @ReactMethod
    public final void feeLinearFee(RPtr constant, RPtr coefficient, RPtr certificate, Promise promise) {
        Native.I
                .feeLinearFee(constant, coefficient, certificate)
                .map(RPtr::toJs)
                .pour(promise);
    }

    @ReactMethod
    public final void ptrFree(String ptr) {
        (new RPtr(ptr)).free();
    }
}

