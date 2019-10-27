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

    public final native Result<RPtr> valueFromStr(String str);
    public final native Result<String> valueToStr(RPtr value);
    public final native Result<RPtr> valueCheckedAdd(RPtr value, RPtr other);
    public final native Result<RPtr> valueCheckedSub(RPtr value, RPtr other);


    public final native Result<RPtr> publicKeyFromBech32(String bech32);
    public final native Result<byte[]> publicKeyAsBytes(RPtr pubKey);

    public final native void ptrFree(RPtr ptr);
}
