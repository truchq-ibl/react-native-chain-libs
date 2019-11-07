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
        return address_account_from_public_key(&key,
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
        RPtr authTx = [ptr rPtr];
        return authenticated_transaction_transaction(authTx, &result, error)
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

RCT_EXPORT_METHOD(feeLinearFee:(nonnull NSString *)constant withCoefficient:(nonnull NSString *)coefficient andCertificate:(nonnull NSString *)certificate withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
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

RCT_EXPORT_METHOD(feeCalculate:(nonnull NSString *)feePtr withTx:(nonnull NSString *)txPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr fee = [[params objectAtIndex:0] rPtr];
        RPtr tx = [[params objectAtIndex:1] rPtr];
        return fee_calculate(fee, &tx, &result, error)
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
        return output_policy_one(&address, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:addressPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionFinalizerNew:(nonnull NSString *)transactionPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* transactionPtr, CharPtr* error) {
        RPtr result;
        RPtr transaction = [transactionPtr rPtr];
        return transaction_finalizer_new(&transaction, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:transactionPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionFinalizerGetTxSignDataHash:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr txFinalizer = [ptr rPtr];
        return transaction_finalizer_get_tx_sign_data_hash(txFinalizer, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionFinalizerSetWitness:(nonnull NSString *)txFinalizer withIndex:(nonnull NSNumber *)index andWitness:(nonnull NSString *)witness withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr txFinalizer = [[params objectAtIndex:0] rPtr];
        RPtr witness = [[params objectAtIndex:2] rPtr];
        transaction_finalizer_set_witness(txFinalizer,
                                          [[params objectAtIndex:1] unsignedLongValue],
                                          &witness,
                                          error);
        return nil;
    }] exec:@[txFinalizer, index, witness] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(transactionFinalizerBuild:(nonnull NSString *)ptr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* ptr, CharPtr* error) {
        RPtr result;
        RPtr txFinalizer = [ptr rPtr];
        return transaction_finalizer_build(&txFinalizer, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(witnessForAccount:(nonnull NSString *)genesisHash withTransactionId:(nonnull NSString *)transactionId andSecretKey:(nonnull NSString *)secretKey andAccountSpendingCounter:(nonnull NSString *)accountSpendingCounter withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr genesisHash = [[params objectAtIndex:0] rPtr];
        RPtr transactionId = [[params objectAtIndex:1] rPtr];
        RPtr secretKey = [[params objectAtIndex:2] rPtr];
        RPtr accountSpendingCounter = [[params objectAtIndex:3] rPtr];
        return witness_for_account(&genesisHash,
                                   &transactionId,
                                   &secretKey,
                                   &accountSpendingCounter,
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
        return witness_for_utxo(&genesisHash,
                                   &transactionId,
                                   &secretKey,
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

RCT_EXPORT_METHOD(fragmentIdFromBytes:(nonnull NSString *)bytesStr  withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[CSafeOperation new:^NSString*(NSString* bytesStr, CharPtr* error) {
        RPtr result;
        NSData* data = [NSData fromBase64:bytesStr];
        return fragment_id_from_bytes((uint8_t*)data.bytes, data.length, &result, error)
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
        return utxo_pointer_new(&fragmentId, outputIndex, &value, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[fragmentIdPtr, outputIndex, valuePtr] andResolve:resolve orReject:reject];
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
