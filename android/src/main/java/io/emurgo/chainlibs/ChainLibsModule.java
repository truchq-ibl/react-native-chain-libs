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

    @ReactMethod
    public final void ptrFree(String ptr) {
        (new RPtr(ptr)).free();
    }


}

