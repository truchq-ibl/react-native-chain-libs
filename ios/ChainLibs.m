#import "ChainLibs.h"
#import "NSString+RPtr.h"
#import "NSData+DataPtr.h"
#import "SafeOperation.h"
#import <react_native_chain_libs.h>

@implementation ChainLibs

RCT_EXPORT_MODULE(ChainLibs)

+ (BOOL)requiresMainQueueSetup {
    return NO;
}

- (NSDictionary *)constantsToExport {
    return @{
             @"AddressDiscrimination": @{
                     @"Test": @(Test),
                     @"Production": @(Production)
                     },
             @"CertificateKind": @{
                     @"StakeDelegation": @(StakeDelegation),
                     @"OwnerStakeDelegation": @(OwnerStakeDelegation),
                     @"PoolRegistration": @(PoolRegistration),
                     @"PoolRetirement": @(PoolRetirement),
                     @"PoolUpdate": @(PoolUpdate)
                     },
             @"AddressKind": @{
                     @"Single": @(Single),
                     @"Group": @(Group),
                     @"Account": @(Account),
                     @"Multisig": @(Multisig)
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

RCT_EXPORT_METHOD(addressFromBytes:(nonnull NSString *)bytesStr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bytesStr, CharPtr* error) {
        RPtr result;
        NSData* data = [NSData fromBase64:bytesStr];
        return address_from_bytes((uint8_t*)data.bytes, data.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesStr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressAsBytes:(nonnull NSString *)addressPtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* addressPtr, CharPtr* error) {
        DataPtr result;
        RPtr address = [addressPtr rPtr];
        return address_as_bytes(address, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:addressPtr andResolve:resolve orReject:reject];
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
        return address_single_from_public_key(key,
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
        return address_delegation_from_public_key(key,
                                                  delegation,
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
        return address_account_from_public_key(key,
                                              [[params objectAtIndex:1] intValue],
                                              &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[key, discrimination] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressGetDiscrimination:(nonnull NSString *)addressPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* addressPtr, CharPtr* error) {
        AddressDiscrimination result;
        RPtr address = [addressPtr rPtr];
        return address_get_discrimination(address, &result, error)
            ? [NSNumber numberWithInt:result]
            : nil;
    }] exec:addressPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressGetKind:(nonnull NSString *)addressPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* addressPtr, CharPtr* error) {
        AddressKind result;
        RPtr address = [addressPtr rPtr];
        return address_get_kind(address, &result, error)
            ? [NSNumber numberWithInt:result]
            : nil;
    }] exec:addressPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressToSingleAddress:(nonnull NSString *)addressPtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* addressPtr, CharPtr* error) {
        RPtr result;
        RPtr address = [addressPtr rPtr];
        return address_to_single_address(address, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:addressPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressToGroupAddress:(nonnull NSString *)addressPtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* addressPtr, CharPtr* error) {
        RPtr result;
        RPtr address = [addressPtr rPtr];
        return address_to_group_address(address, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:addressPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(addressToAccountAddress:(nonnull NSString *)addressPtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* addressPtr, CharPtr* error) {
        RPtr result;
        RPtr address = [addressPtr rPtr];
        return address_to_account_address(address, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:addressPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentFromTransaction:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr tx = [ptr rPtr];
        return fragment_from_transaction(tx, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentGetTransaction:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr fragment = [ptr rPtr];
        return fragment_get_transaction(fragment, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentAsBytes:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        DataPtr result;
        RPtr fragment = [ptr rPtr];
        return fragment_as_bytes(fragment, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
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

RCT_EXPORT_METHOD(fragmentIsPoolRetirement:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_pool_retirement(fragment, &result, error)
            ? [NSNumber numberWithBool:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIsPoolUpdate:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* ptr, CharPtr* error) {
        BOOL result;
        RPtr fragment = [ptr rPtr];
        return fragment_is_pool_update(fragment, &result, error)
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

RCT_EXPORT_METHOD(transactionBuilderNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return transaction_builder_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderPayload:(nonnull NSString *)txBuilderPtr withCert:(nonnull NSString *)certPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr txBuilder = [[params objectAtIndex:0] rPtr];
        RPtr cert = [[params objectAtIndex:1] rPtr];
        return transaction_builder_payload(&txBuilder, cert, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[txBuilderPtr, certPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderNoPayload:(nonnull NSString *)txBuilderPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* txBuilderPtr, CharPtr* error) {
        RPtr result;
        RPtr txBuilder = [txBuilderPtr rPtr];
        return transaction_builder_no_payload(&txBuilder, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:txBuilderPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderSetIOsSetIOs:(nonnull NSString *)txBuilderSetIOsPtr withInputs:(nonnull NSString *)inputsPtr andOutputs:(nonnull NSString *)outputsPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr txBuilderSetIOs = [[params objectAtIndex:0] rPtr];
        RPtr inputs = [[params objectAtIndex:1] rPtr];
        RPtr outputs = [[params objectAtIndex:2] rPtr];
        return transaction_builder_set_ios_set_ios(&txBuilderSetIOs, inputs, outputs, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[txBuilderSetIOsPtr, inputsPtr, outputsPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderSetWitnessGetAuthDataForWitness:(nonnull NSString *)txBuilderSetWitnessPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* txBuilderSetWitnessPtr, CharPtr* error) {
        RPtr result;
        RPtr txBuilderSetWitness = [txBuilderSetWitnessPtr rPtr];
        return transaction_builder_set_witness_get_auth_data_for_witness(txBuilderSetWitness,
                                                                         &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:txBuilderSetWitnessPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderSetWitnessSetWitnesses:(nonnull NSString *)txBuilderSetWitnessPtr withWitnesses:(nonnull NSString *)witnessesPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr txBuilderSetWitness = [[params objectAtIndex:0] rPtr];
        RPtr witnesses = [[params objectAtIndex:1] rPtr];
        return transaction_builder_set_witness_set_witnesses(&txBuilderSetWitness, witnesses, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[txBuilderSetWitnessPtr, witnessesPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderSetAuthDataGetAuthData:(nonnull NSString *)txBuilderSetAuthDataPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* txBuilderSetAuthDataPtr, CharPtr* error) {
        RPtr result;
        RPtr txBuilderSetAuthData = [txBuilderSetAuthDataPtr rPtr];
        return transaction_builder_set_auth_data_get_auth_data(txBuilderSetAuthData, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:txBuilderSetAuthDataPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionBuilderSetAuthDataSetPayloadAuth:(nonnull NSString *)txBuilderSetAuthDataPtr withAuth:(nonnull NSString *)authPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr txBuilderSetAuthData = [[params objectAtIndex:0] rPtr];
        RPtr auth = [[params objectAtIndex:1] rPtr];
        return transaction_builder_set_auth_data_set_payload_auth(&txBuilderSetAuthData, auth, &result, error)
        ? [NSString stringFromPtr:result]
        : nil;
    }] exec:@[txBuilderSetAuthDataPtr, authPtr] andResolve:resolve orReject:reject];
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

RCT_EXPORT_METHOD(accountSingleFromPublicKey:(nonnull NSString *)keyPtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* keyPtr, CharPtr* error) {
        RPtr result;
        RPtr key = [keyPtr rPtr];
        return account_single_from_public_key(key, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:keyPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputFromAccount:(nonnull NSString *)account withV:(nonnull NSString *)v withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr account = [[params objectAtIndex:0] rPtr];
        RPtr v = [[params objectAtIndex:1] rPtr];
        return input_from_account(account, v, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[account, v] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputValue:(nonnull NSString *)inputPtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* inputPtr, CharPtr* error) {
        RPtr result;
        RPtr input = [inputPtr rPtr];
        return input_value(input, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:inputPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputsNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return inputs_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputsSize:(nonnull NSString *)inputsPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* inputsPtr, CharPtr* error) {
        uintptr_t result;
        RPtr inputs = [inputsPtr rPtr];
        return inputs_size(inputs, &result, error)
            ? [NSNumber numberWithUnsignedLong:result]
            : nil;
    }] exec:inputsPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputsGet:(nonnull NSString *)inputsPtr withIndex:(nonnull NSNumber *)index withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr inputs = [[params objectAtIndex:0] rPtr];
        uintptr_t index = [[params objectAtIndex:1] unsignedIntegerValue];
        return inputs_get(inputs, index, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[inputsPtr, index] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputsAdd:(nonnull NSString *)inputsPtr withItem:(nonnull NSString *)item withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr inputs = [[params objectAtIndex:0] rPtr];
        RPtr item = [[params objectAtIndex:1] rPtr];
        inputs_add(inputs, &item, error);
        return nil;
    }] exec:@[inputsPtr, item] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(feeLinearFee:(nonnull NSString *)constant withCoefficient:(nonnull NSString *)coefficient andCertificate:(nonnull NSString *)certificate withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr constant = [[params objectAtIndex:0] rPtr];
        RPtr coefficient = [[params objectAtIndex:1] rPtr];
        RPtr certificate = [[params objectAtIndex:2] rPtr];
        return fee_linear_fee(constant, coefficient, certificate, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[constant, coefficient, certificate] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(feeCalculate:(nonnull NSString *)feePtr withTx:(nonnull NSString *)txPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr fee = [[params objectAtIndex:0] rPtr];
        RPtr tx = [[params objectAtIndex:1] rPtr];
        return fee_calculate(fee, tx, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[feePtr, txPtr] andResolve:resolve orReject:reject];
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
        return output_policy_one(address, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:addressPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(witnessForAccount:(nonnull NSString *)genesisHash withTransactionId:(nonnull NSString *)transactionId andSecretKey:(nonnull NSString *)secretKey andAccountSpendingCounter:(nonnull NSString *)accountSpendingCounter withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr genesisHash = [[params objectAtIndex:0] rPtr];
        RPtr transactionId = [[params objectAtIndex:1] rPtr];
        RPtr secretKey = [[params objectAtIndex:2] rPtr];
        RPtr accountSpendingCounter = [[params objectAtIndex:3] rPtr];
        return witness_for_account(genesisHash,
                                   transactionId,
                                   secretKey,
                                   accountSpendingCounter,
                                   &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[genesisHash, transactionId, secretKey, accountSpendingCounter] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(witnessForUtxo:(nonnull NSString *)genesisHash withTransactionId:(nonnull NSString *)transactionId andSecretKey:(nonnull NSString *)secretKey withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr genesisHash = [[params objectAtIndex:0] rPtr];
        RPtr transactionId = [[params objectAtIndex:1] rPtr];
        RPtr secretKey = [[params objectAtIndex:2] rPtr];
        return witness_for_utxo(genesisHash,
                                transactionId,
                                secretKey,
                                &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[genesisHash, transactionId, secretKey] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(witnessForLegacyIcarusUtxo:(nonnull NSString *)genesisHash withTransactionId:(nonnull NSString *)transactionId andSecretKey:(nonnull NSString *)secretKey withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr genesisHash = [[params objectAtIndex:0] rPtr];
        RPtr transactionId = [[params objectAtIndex:1] rPtr];
        RPtr secretKey = [[params objectAtIndex:2] rPtr];
        return witness_for_legacy_icarus_utxo(genesisHash,
                                transactionId,
                                secretKey,
                                &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[genesisHash, transactionId, secretKey] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(privateKeyFromBech32:(nonnull NSString *)bech32Str withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bech32Str, CharPtr* error) {
        RPtr result;
        return private_key_from_bech32([bech32Str charPtr], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bech32Str andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(privateKeyToPublic: (nonnull NSString *)ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        return private_key_to_public([ptr rPtr], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(privateKeyFromExtendedBytes:(nonnull NSString *)bytesStr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bytesStr, CharPtr* error) {
        RPtr result;
        NSData* data = [NSData fromBase64:bytesStr];
        return private_key_from_extended_bytes((uint8_t*)data.bytes, data.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesStr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(hashFromHex:(nonnull NSString *)hexString withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* hexString, CharPtr* error) {
        RPtr result;
        return hash_from_hex([hexString charPtr], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:hexString andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(spendingCounterZero:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return spending_counter_zero(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(spendingCounterFromU32:(nonnull NSNumber *)counter withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSNumber* counter, CharPtr* error) {
        RPtr result;
        return spending_counter_from_u32([counter intValue], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:counter andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionId:(nonnull NSString *)ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr transaction = [ptr rPtr];
        return transaction_id(transaction, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionInputs:(nonnull NSString *)ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr transaction = [ptr rPtr];
        return transaction_inputs(transaction, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionOutputs:(nonnull NSString *)ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr transaction = [ptr rPtr];
        return transaction_outputs(transaction, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(outputAddress:(nonnull NSString *)outputPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* outputPtr, CharPtr* error) {
        RPtr result;
        RPtr output = [outputPtr rPtr];
        return output_address(output, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:outputPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(outputValue:(nonnull NSString *)outputPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* outputPtr, CharPtr* error) {
        RPtr result;
        RPtr output = [outputPtr rPtr];
        return output_value(output, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:outputPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(outputsNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return outputs_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(outputsSize:(nonnull NSString *)outputsPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* outputsPtr, CharPtr* error) {
        uintptr_t result;
        RPtr outputs = [outputsPtr rPtr];
        return outputs_size(outputs, &result, error)
            ? [NSNumber numberWithUnsignedLong:result]
            : nil;
    }] exec:outputsPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(outputsGet:(nonnull NSString *)outputsPtr withIndex:(nonnull NSNumber *)index withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr outputs = [[params objectAtIndex:0] rPtr];
        uintptr_t index = [[params objectAtIndex:1] unsignedIntegerValue];
        return outputs_get(outputs, index, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[outputsPtr, index] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(outputsAdd:(nonnull NSString *)outputsPtr withItem:(nonnull NSString *)item withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr outputs = [[params objectAtIndex:0] rPtr];
        RPtr item = [[params objectAtIndex:1] rPtr];
        outputs_add(outputs, &item, error);
        return nil;
    }] exec:@[outputsPtr, item] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIdCalculate:(nonnull NSString *)bytesStr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bytesStr, CharPtr* error) {
        RPtr result;
        NSData* data = [NSData fromBase64:bytesStr];
        return fragment_id_calculate((uint8_t*)data.bytes, data.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesStr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(fragmentIdAsBytes:(nonnull NSString *)fragmentIdPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* fragmentIdPtr, CharPtr* error) {
        DataPtr result;
        RPtr fragment_id = [fragmentIdPtr rPtr];
        return fragment_id_as_bytes(fragment_id, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:fragmentIdPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionSignDataHashFromBytes:(nonnull NSString *)bytesStr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bytesStr, CharPtr* error) {
        RPtr result;
        NSData* data = [NSData fromBase64:bytesStr];
        return transaction_sign_data_hash_from_bytes((uint8_t*)data.bytes, data.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesStr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionSignDataHashFromHex:(nonnull NSString *)input withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* input, CharPtr* error) {
        RPtr result;
        return transaction_sign_data_hash_from_hex([input charPtr], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:input andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionSignDataHashAsBytes:(nonnull NSString *)txSignDataHashPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* txSignDataHashPtr, CharPtr* error) {
        DataPtr result;
        RPtr txSignDataHash = [txSignDataHashPtr rPtr];
        return transaction_sign_data_hash_as_bytes(txSignDataHash, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:txSignDataHashPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(utxoPointerNew:(nonnull NSString *)fragmentIdPtr withOutputIndex:(nonnull NSNumber *)outputIndex withValue:(nonnull NSString *)valuePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr fragmentId = [[params objectAtIndex:0] rPtr];
        uint8_t outputIndex = [[params objectAtIndex:1] intValue];
        RPtr value = [[params objectAtIndex:2] rPtr];
        return utxo_pointer_new(fragmentId, outputIndex, value, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[fragmentIdPtr, outputIndex, valuePtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(balanceIsPositive:(nonnull NSString *)balancePtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* balancePtr, CharPtr* error) {
        BOOL result;
        RPtr balance = [balancePtr rPtr];
        return balance_is_positive(balance, &result, error)
            ? [NSNumber numberWithBool:result]
            : nil;
    }] exec:balancePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(balanceIsNegative:(nonnull NSString *)balancePtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* balancePtr, CharPtr* error) {
        BOOL result;
        RPtr balance = [balancePtr rPtr];
        return balance_is_negative(balance, &result, error)
            ? [NSNumber numberWithBool:result]
            : nil;
    }] exec:balancePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(balanceIsZero:(nonnull NSString *)balancePtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* balancePtr, CharPtr* error) {
        BOOL result;
        RPtr balance = [balancePtr rPtr];
        return balance_is_zero(balance, &result, error)
            ? [NSNumber numberWithBool:result]
            : nil;
    }] exec:balancePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(balanceGetValue:(nonnull NSString *)balancePtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* balancePtr, CharPtr* error) {
        RPtr result;
        RPtr balance = [balancePtr rPtr];
        return balance_get_value(balance, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:balancePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputOutputBuilderEmpty:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _value, CharPtr* error) {
        RPtr result;
        return input_output_builder_empty(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputOutputBuilderAddInput:(nonnull NSString *)ioBuilderPtr withInput:(nonnull NSString *)inputPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr ioBuilder = [[params objectAtIndex:0] rPtr];
        RPtr input = [[params objectAtIndex:1] rPtr];
        input_output_builder_add_input(ioBuilder, input, error);
        return nil;
    }] exec:@[ioBuilderPtr, inputPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputOutputBuilderAddOutput:(nonnull NSString *)ioBuilderPtr withAddress:(nonnull NSString *)addressPtr andValue:(nonnull NSString *)valuePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr ioBuilder = [[params objectAtIndex:0] rPtr];
        RPtr address = [[params objectAtIndex:1] rPtr];
        RPtr value = [[params objectAtIndex:2] rPtr];
        input_output_builder_add_output(ioBuilder, address, value, error);
        return nil;
    }] exec:@[ioBuilderPtr, addressPtr, valuePtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputOutputBuilderEstimateFee:(nonnull NSString *)ioBuilderPtr withFee:(nonnull NSString *)feePtr andPayload:(nonnull NSString *)payloadPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr ioBuilder = [[params objectAtIndex:0] rPtr];
        RPtr fee = [[params objectAtIndex:1] rPtr];
        RPtr payload = [[params objectAtIndex:2] rPtr];
        return input_output_builder_estimate_fee(ioBuilder, fee, payload, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[ioBuilderPtr, feePtr, payloadPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputOutputBuilderBuild:(nonnull NSString *)ioBuilderPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ioBuilderPtr, CharPtr* error) {
        RPtr result;
        RPtr ioBuilder = [ioBuilderPtr rPtr];
        return input_output_builder_build(&ioBuilder, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ioBuilderPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputOutputBuilderSealWithOutputPolicy:(nonnull NSString *)ioBuilderPtr withPayload:(nonnull NSString *)payloadPtr andFeeAlgorithm:(nonnull NSString *)feeAlgorithmPtr andPolicy:(nonnull NSString *)policyPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr ioBuilder = [[params objectAtIndex:0] rPtr];
        RPtr payload = [[params objectAtIndex:1] rPtr];
        RPtr feeAlgorithm = [[params objectAtIndex:2] rPtr];
        RPtr policy = [[params objectAtIndex:3] rPtr];
        return input_output_builder_seal_with_output_policy(&ioBuilder, payload, feeAlgorithm, policy, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[ioBuilderPtr, payloadPtr, feeAlgorithmPtr, policyPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputOutputInputs:(nonnull NSString *)inputOutputPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* inputOutputPtr, CharPtr* error) {
        RPtr result;
        RPtr inputOutput = [inputOutputPtr rPtr];
        return input_output_inputs(inputOutput, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:inputOutputPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(inputOutputOutputs:(nonnull NSString *)inputOutputPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* inputOutputPtr, CharPtr* error) {
        RPtr result;
        RPtr inputOutput = [inputOutputPtr rPtr];
        return input_output_outputs(inputOutput, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:inputOutputPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(witnessesNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return witnesses_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(witnessesSize:(nonnull NSString *)witnessesPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* witnessesPtr, CharPtr* error) {
        uintptr_t result;
        RPtr witnesses = [witnessesPtr rPtr];
        return witnesses_size(witnesses, &result, error)
            ? [NSNumber numberWithUnsignedLong:result]
            : nil;
    }] exec:witnessesPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(witnessesGet:(nonnull NSString *)witnessesPtr withIndex:(nonnull NSNumber *)index withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr witnesses = [[params objectAtIndex:0] rPtr];
        uintptr_t index = [[params objectAtIndex:1] unsignedIntegerValue];
        return witnesses_get(witnesses, index, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[witnessesPtr, index] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(witnessesAdd:(nonnull NSString *)witnessesPtr withItem:(nonnull NSString *)item withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr witnesses = [[params objectAtIndex:0] rPtr];
        RPtr item = [[params objectAtIndex:1] rPtr];
        witnesses_add(witnesses, &item, error);
        return nil;
    }] exec:@[witnessesPtr, item] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(payloadAuthDataForNoPayload:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return payload_auth_data_for_no_payload(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(payloadNoPayload:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return payload_no_payload(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(stakeDelegationAuthDataNew:(nonnull NSString *)signaturePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* signaturePtr, CharPtr* error) {
        RPtr result;
        RPtr signature = [signaturePtr rPtr];
        return stake_delegation_auth_data_new(signature, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:signaturePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(stakeDelegationNew:(nonnull NSString *)delegationTypePtr withAccount:(nonnull NSString *)accountPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr delegationType = [[params objectAtIndex:0] rPtr];
        RPtr account = [[params objectAtIndex:1] rPtr];
        return stake_delegation_new(delegationType, account, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[delegationTypePtr, accountPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(stakeDelegationDelegationType:(nonnull NSString *)stakeDelegationPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* stakeDelegationPtr, CharPtr* error) {
        RPtr result;
        RPtr stakeDelegation = [stakeDelegationPtr rPtr];
        return stake_delegation_delegation_type(stakeDelegation, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:stakeDelegationPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(stakeDelegationAccount:(nonnull NSString *)stakeDelegationPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* stakeDelegationPtr, CharPtr* error) {
        RPtr result;
        RPtr stakeDelegation = [stakeDelegationPtr rPtr];
        return stake_delegation_account(stakeDelegation, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:stakeDelegationPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(stakeDelegationAsBytes:(nonnull NSString *)stakeDelegationPtr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* stakeDelegationPtr, CharPtr* error) {
        DataPtr result;
        RPtr stakeDelegation = [stakeDelegationPtr rPtr];
        return stake_delegation_as_bytes(stakeDelegation, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:stakeDelegationPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(stakeDelegationFromBytes:(nonnull NSString *)bytes  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bytes, CharPtr* error) {
        RPtr result;
        NSData* data = [NSData fromBase64:bytes];
        return stake_delegation_from_bytes((uint8_t*)data.bytes, data.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytes andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(certificateStakeDelegation:(nonnull NSString *)stakeDelegationPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* stakeDelegationPtr, CharPtr* error) {
        RPtr result;
        RPtr stakeDelegation = [stakeDelegationPtr rPtr];
        return certificate_stake_delegation(stakeDelegation, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:stakeDelegationPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(certificateStakePoolRegistration:(nonnull NSString *)poolRegistrationPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* poolRegistrationPtr, CharPtr* error) {
        RPtr result;
        RPtr poolRegistration = [poolRegistrationPtr rPtr];
        return certificate_stake_pool_registration(poolRegistration, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:poolRegistrationPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(certificateStakePoolRetirement:(nonnull NSString *)poolRetirementPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* poolRetirementPtr, CharPtr* error) {
        RPtr result;
        RPtr poolRetirement = [poolRetirementPtr rPtr];
        return certificate_stake_pool_retirement(poolRetirement, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:poolRetirementPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(certificateGetType:(nonnull NSString *)certificatePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSNumber*(NSString* certificatePtr, CharPtr* error) {
        CertificateKind result;
        RPtr certificate = [certificatePtr rPtr];
        return certificate_get_type(certificate, &result, error)
            ? [NSNumber numberWithInt:result]
            : nil;
    }] exec:certificatePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(certificateGetStakeDelegation:(nonnull NSString *)certificatePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* certificatePtr, CharPtr* error) {
        RPtr result;
        RPtr certificate = [certificatePtr rPtr];
        return certificate_get_stake_delegation(certificate, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:certificatePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(certificateGetOwnerStakeDelegation:(nonnull NSString *)certificatePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* certificatePtr, CharPtr* error) {
        RPtr result;
        RPtr certificate = [certificatePtr rPtr];
        return certificate_get_owner_stake_delegation(certificate, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:certificatePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(certificateGetPoolRegistration:(nonnull NSString *)certificatePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* certificatePtr, CharPtr* error) {
        RPtr result;
        RPtr certificate = [certificatePtr rPtr];
        return certificate_get_pool_registration(certificate, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:certificatePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(certificateGetPoolRetirement:(nonnull NSString *)certificatePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* certificatePtr, CharPtr* error) {
        RPtr result;
        RPtr certificate = [certificatePtr rPtr];
        return certificate_get_pool_retirement(certificate, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:certificatePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(accountBindingSignatureNewSingle:(nonnull NSString *)privateKeyPtr withAuthData:(nonnull NSString *)authDataPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr privateKey = [[params objectAtIndex:0] rPtr];
        RPtr authData = [[params objectAtIndex:1] rPtr];
        return account_binding_signature_new_single(privateKey, authData, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[privateKeyPtr, authDataPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PrivateKeyDerive:(nonnull NSString *)bip32PrivateKeyPtr withIndex:(nonnull NSNumber *)index withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr bip32PrivateKey = [[params objectAtIndex:0] rPtr];
        uint32_t index = [[params objectAtIndex:1] intValue];
        return bip_32_private_key_derive(bip32PrivateKey, index, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[bip32PrivateKeyPtr, index] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PrivateKeyGenerateEd25519Bip32:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return bip_32_private_key_generate_ed25519_bip32(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PrivateKeyToRawKey:(nonnull NSString *)bip32PrivateKeyPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bip32PrivateKeyPtr, CharPtr* error) {
        RPtr result;
        RPtr bip32PrivateKey = [bip32PrivateKeyPtr rPtr];
        return bip_32_private_key_to_raw_key(bip32PrivateKey, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bip32PrivateKeyPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PrivateKeyToPublic:(nonnull NSString *)bip32PrivateKeyPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bip32PrivateKeyPtr, CharPtr* error) {
        RPtr result;
        RPtr bip32PrivateKey = [bip32PrivateKeyPtr rPtr];
        return bip_32_private_key_to_public(bip32PrivateKey, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bip32PrivateKeyPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PrivateKeyFromBytes:(nonnull NSString *)bytesStr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bytesStr, CharPtr* error) {
        RPtr result;
        NSData* data = [NSData fromBase64:bytesStr];
        return bip_32_private_key_from_bytes((uint8_t*)data.bytes, data.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesStr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PrivateKeyAsBytes:(nonnull NSString *)bip32PrivateKeyPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bip32PrivateKeyPtr, CharPtr* error) {
        DataPtr result;
        RPtr bip32PrivateKey = [bip32PrivateKeyPtr rPtr];
        return bip_32_private_key_as_bytes(bip32PrivateKey, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:bip32PrivateKeyPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PrivateKeyFromBech32:(nonnull NSString *)bech32Str withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bech32Str, CharPtr* error) {
        RPtr result;
        return bip_32_private_key_from_bech32([bech32Str charPtr], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bech32Str andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PrivateKeyToBech32:(nonnull NSString *)bip32PrivateKeyPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bip32PrivateKeyPtr, CharPtr* error) {
        CharPtr result;
        RPtr bip32PrivateKey = [bip32PrivateKeyPtr rPtr];
        return bip_32_private_key_to_bech32(bip32PrivateKey, &result, error)
            ? [NSString stringFromCharPtr:&result]
            : nil;
    }] exec:bip32PrivateKeyPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PrivateKeyFromBip39Entropy:(nonnull NSString *)entropy withPassword:(nonnull NSString *)password withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        NSData* entropy = [NSData fromBase64:[params objectAtIndex:0]];
        NSData* password = [NSData fromBase64:[params objectAtIndex:1]];
        return bip_32_private_key_from_bip39_entropy(
                                                     (uint8_t*)entropy.bytes, entropy.length,
                                                     (uint8_t*)password.bytes, password.length,
                                                     &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[entropy, password] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PublicKeyDerive:(nonnull NSString *)bip32PublicKeyPtr withIndex:(nonnull NSNumber *)index withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr bip32PublicKey = [[params objectAtIndex:0] rPtr];
        uint32_t index = [[params objectAtIndex:1] intValue];
        return bip32_public_key_derive(bip32PublicKey, index, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[bip32PublicKeyPtr, index] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PublicKeyToRawKey:(nonnull NSString *)bip32PublicKeyPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bip32PublicKeyPtr, CharPtr* error) {
        RPtr result;
        RPtr bip32PublicKey = [bip32PublicKeyPtr rPtr];
        return bip32_public_key_to_raw_key(bip32PublicKey, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bip32PublicKeyPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PublicKeyFromBytes:(nonnull NSString *)bytesStr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bytesStr, CharPtr* error) {
        RPtr result;
        NSData* data = [NSData fromBase64:bytesStr];
        return bip32_public_key_from_bytes((uint8_t*)data.bytes, data.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesStr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PublicKeyAsBytes:(nonnull NSString *)bip32PublicKeyPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bip32PublicKeyPtr, CharPtr* error) {
        DataPtr result;
        RPtr bip32PublicKey = [bip32PublicKeyPtr rPtr];
        return bip32_public_key_as_bytes(bip32PublicKey, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:bip32PublicKeyPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PublicKeyFromBech32:(nonnull NSString *)bech32Str withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bech32Str, CharPtr* error) {
        RPtr result;
        return bip32_public_key_from_bech32([bech32Str charPtr], &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bech32Str andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(bip32PublicKeyToBech32:(nonnull NSString *)bip32PublicKeyPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bip32PrivateKeyPtr, CharPtr* error) {
        CharPtr result;
        RPtr bip32PublicKey = [bip32PublicKeyPtr rPtr];
        return bip32_public_key_to_bech32(bip32PublicKey, &result, error)
            ? [NSString stringFromCharPtr:&result]
            : nil;
    }] exec:bip32PublicKeyPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(delegationTypeNonDelegated:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return delegation_type_non_delegated(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(delegationTypeFull:(nonnull NSString *)poolIdPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* poolIdPtr, CharPtr* error) {
        RPtr result;
        RPtr poolId = [poolIdPtr rPtr];
        return delegation_type_full(poolId, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:poolIdPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(delegationTypeRatio:(nonnull NSString *)rPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* rPtr, CharPtr* error) {
        RPtr result;
        RPtr r = [rPtr rPtr];
        return delegation_type_ratio(r, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:rPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(delegationTypeGetKind:(nonnull NSString *)delegationTypePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* delegationTypePtr, CharPtr* error) {
        RPtr result;
        RPtr delegationType = [delegationTypePtr rPtr];
        return delegation_type_get_kind(delegationType, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:delegationTypePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(delegationTypeGetFull:(nonnull NSString *)delegationTypePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* delegationTypePtr, CharPtr* error) {
        RPtr result;
        RPtr delegationType = [delegationTypePtr rPtr];
        return delegation_type_get_full(delegationType, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:delegationTypePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(ptrFree:(NSString *)ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    RPtr rPtr = [ptr rPtr];
    rptr_free(&rPtr);
    resolve(nil);
}

+ (void)initialize
{
    if (self == [ChainLibs class]) {
        init_chain_libs_library();
    }
}

@end
