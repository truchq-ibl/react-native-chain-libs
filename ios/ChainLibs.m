#import "ChainLibs.h"
#import "NSString+RPtr.h"
#import "NSData+DataPtr.h"
#import "SafeOperation.h"
#import <react_native_chain_libs.h>

@implementation ChainLibs

RCT_EXPORT_MODULE(ChainLibs)

RCT_EXPORT_METHOD(valueFromStr:(nonnull NSString *)string withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* string, CharPtr* error) {
        RPtr result;
        return value_from_str([string charPtr], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:string andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(valueToStr:(nonnull NSString *)ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        CharPtr result;
        return value_to_str([ptr rPtr], &result, error)
            ? [NSString stringFromCharPtr:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(valueCheckedAdd:(nonnull NSString *)ptr1 other:(nonnull NSString *)ptr2 withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray<NSString*>* ptrs, CharPtr* error) {
        RPtr result;
        return value_checked_add([[ptrs objectAtIndex:0] rPtr],
                                 [[ptrs objectAtIndex:1] rPtr],
                                 &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[ptr1, ptr2] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(valueCheckedSub:(nonnull NSString *)ptr1 other:(nonnull NSString *)ptr2 withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray<NSString*>* ptrs, CharPtr* error) {
        RPtr result;
        return value_checked_sub([[ptrs objectAtIndex:0] rPtr],
                                 [[ptrs objectAtIndex:1] rPtr],
                                 &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[ptr1, ptr2] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(publicKeyFromBech32:(nonnull NSString *)bech32_str withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* string, CharPtr* error) {
        RPtr result;
        return public_key_from_bech32([string charPtr], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bech32_str andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(publicKeyAsBytes:(nonnull NSString *)ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        DataPtr result;
        return public_key_as_bytes([ptr rPtr], &result, error)
            ? [[NSData fromDataPtr:result] base64]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressFromString:(nonnull NSString *)string withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* string, CharPtr* error) {
        RPtr result;
        return address_from_string([string charPtr], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:string andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressToString:(nonnull NSString *)ptr withPrefix:(nonnull NSString *)prefix withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray<NSString*>* params, CharPtr* error) {
        CharPtr result;
        return address_to_string([[params objectAtIndex:0] rPtr], [[params objectAtIndex:1] charPtr], &result, error)
            ? [NSString stringFromCharPtr:result]
            : nil;
    }] exec:@[ptr, prefix] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressSingleFromPublicKey:(nonnull NSString *)pubPtr withDiscrimination:(nonnull NSNumber *)discrimination withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        return address_single_from_public_key([[params objectAtIndex:0] rPtr],
                                              [[params objectAtIndex:1] intValue],
                                              &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[pubPtr, discrimination] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(ptrFree:(NSString *)ptr)
{
    rptr_free([ptr rPtr]);
}

+ (void)initialize
{
    if (self == [ChainLibs class]) {
        init_chain_libs_library();
    }
}

@end
