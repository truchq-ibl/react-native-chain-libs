#import "ChainLibs.h"
#import "NSString+RPtr.h"
#import "NSData+DataPtr.h"
#import "SafeOperation.h"
#import <react_native_chain_libs.h>

@implementation ChainLibs

RCT_EXPORT_MODULE(ChainLibs)

- (NSDictionary *)constantsToExport {
    return @{
             @"AddressDiscrimination": @{
                     @"Test": @(Test),
                     @"Production": @(Production)
                     }
             };
}

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
            ? [NSString stringFromCharPtr:&result]
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
            ? [[NSData fromDataPtr:&result] base64]
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
            ? [NSString stringFromCharPtr:&result]
            : nil;
    }] exec:@[ptr, prefix] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressSingleFromPublicKey:(nonnull NSString *)key withDiscrimination:(nonnull NSNumber *)discrimination withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr key = [[params objectAtIndex:0] rPtr];
        return address_single_from_public_key(&key,
                                              [[params objectAtIndex:1] intValue],
                                              &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[key, discrimination] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressDelegationFromPublicKey:(nonnull NSString *)key withDelegation:(nonnull NSString *)delegation withDiscrimination:(nonnull NSNumber *)discrimination withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr key = [[params objectAtIndex:0] rPtr];
        RPtr delegation = [[params objectAtIndex:1] rPtr];
        return address_delegation_from_public_key(&key,
                                                  &delegation,
                                              [[params objectAtIndex:2] intValue],
                                              &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:@[key, delegation, discrimination] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressAccountFromPublicKey:(nonnull NSString *)key withDiscrimination:(nonnull NSNumber *)discrimination withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr key = [[params objectAtIndex:0] rPtr];
        return address_single_from_public_key(&key,
                                              [[params objectAtIndex:1] intValue],
                                              &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:@[key, discrimination] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentFromAuthenticatedTransaction:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr tx = [ptr rPtr];
        return fragment_from_authenticated_transaction(&tx, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentFromGeneratedTransaction:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr tx = [ptr rPtr];
        return fragment_from_generated_transaction(&tx, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentGetTransaction:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr fragment = [ptr rPtr];
        return fragment_get_transaction(&fragment, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentAsBytes:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSData*(NSString* ptr, CharPtr* error) {
        DataPtr result;
        RPtr fragment = [ptr rPtr];
        return fragment_as_bytes(fragment, &result, error)
        ? [NSData fromDataPtr:&result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsInitial:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_initial(fragment, &result, error)
        ? [NSNumber numberWithBool:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsTransaction:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_transaction(fragment, &result, error)
        ? [NSNumber numberWithBool:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsOwnerStakeDelegation:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_owner_stake_delegation(fragment, &result, error)
        ? [NSNumber numberWithBool:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsStakeDelegation:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_stake_delegation(fragment, &result, error)
        ? [NSNumber numberWithBool:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsPoolRegistration:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_pool_registration(fragment, &result, error)
        ? [NSNumber numberWithBool:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsPoolManagement:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_pool_management(fragment, &result, error)
        ? [NSNumber numberWithBool:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsOldUtxoDeclaration:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_old_utxo_declaration(fragment, &result, error)
        ? [NSNumber numberWithBool:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsUpdateProposal:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_update_proposal(fragment, &result, error)
        ? [NSNumber numberWithBool:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsUpdateVote:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_update_vote(fragment, &result, error)
        ? [NSNumber numberWithBool:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentId:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr fragment = [ptr rPtr];
        return fragment_id(fragment, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(authenticatedTransactionTransaction:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr auth_tx = [ptr rPtr];
        return authenticated_transaction_transaction(auth_tx, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderNewNoPayload:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return transaction_builder_new_no_payload(&result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderAddInput:(nonnull NSString *)ptr withInput:(nonnull NSString *)input andResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr txBuilder = [[params objectAtIndex:0] rPtr];
        RPtr input = [[params objectAtIndex:1] rPtr];
        transaction_builder_add_input(txBuilder, &input, error);
        return nil;
    }] exec:@[ptr, input] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderAddOutput:(nonnull NSString *)ptr withAddress:(nonnull NSString *)address withValue:(nonnull NSString *)value andResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr txBuilder = [[params objectAtIndex:0] rPtr];
        RPtr address = [[params objectAtIndex:1] rPtr];
        RPtr value = [[params objectAtIndex:2] rPtr];
        transaction_builder_add_output(txBuilder, &address, &value, error);
        return nil;
    }] exec:@[ptr, address, value] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderSealWithOutputPolicy:(nonnull NSString *)txBuilder withFee:(nonnull NSString *)fee andOutputPolicy:(nonnull NSString *)outputPolicy withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr txBuilder = [[params objectAtIndex:0] rPtr];
        RPtr fee = [[params objectAtIndex:1] rPtr];
        RPtr outputPolicy = [[params objectAtIndex:2] rPtr];
        return transaction_builder_seal_with_output_policy(&txBuilder, fee, &outputPolicy, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:@[txBuilder, fee, outputPolicy] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(accountFromAddress:(nonnull NSString *)address  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr address = [ptr rPtr];
        return account_from_address(address, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:address andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputFromAccount:(nonnull NSString *)account withV:(nonnull NSString *)v withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr account = [[params objectAtIndex:0] rPtr];
        RPtr v = [[params objectAtIndex:1] rPtr];
        return input_from_account(account, &v, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:@[account, v] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(feeLinearFee:(nonnull NSString *)constant withСoefficient:(nonnull NSString *)coefficient andСertificate:(nonnull NSString *)certificate withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr constant = [[params objectAtIndex:0] rPtr];
        RPtr coefficient = [[params objectAtIndex:1] rPtr];
        RPtr certificate = [[params objectAtIndex:2] rPtr];
        return fee_linear_fee(&constant, &coefficient, &certificate, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:@[constant, coefficient, certificate] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(outputPolicyForget:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return output_policy_forget(&result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(outputPolicyOne:(nonnull NSString *)addressPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* addressPtr, CharPtr* error) {
        RPtr result;
        RPtr address = [addressPtr rPtr];
        return output_policy_one(&address, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:addressPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(ptrFree:(NSString *)ptr)
{
    RPtr rPtr = [ptr rPtr];
    rptr_free(&rPtr);
}

+ (void)initialize
{
    if (self == [ChainLibs class]) {
        init_chain_libs_library();
    }
}

@end
