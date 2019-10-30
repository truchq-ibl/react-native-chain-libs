# react-native-chain-libs

## Getting started

`$ npm install react-native-chain-libs --save`

### Mostly automatic installation

`$ react-native link react-native-chain-libs`

### Manual installation


#### iOS

1. In XCode, in the project navigator, right click `Libraries` ➜ `Add Files to [your project's name]`
2. Go to `node_modules` ➜ `react-native-chain-libs` and add `ChainLibs.xcodeproj`
3. In XCode, in the project navigator, select your project. Add `libChainLibs.a` to your project's `Build Phases` ➜ `Link Binary With Libraries`
4. Run your project (`Cmd+R`)<

#### Android

1. Open up `android/app/src/main/java/[...]/MainApplication.java`
  - Add `import com.reactlibrary.ChainLibsPackage;` to the imports at the top of the file
  - Add `new ChainLibsPackage()` to the list returned by the `getPackages()` method
2. Append the following lines to `android/settings.gradle`:
  	```
  	include ':react-native-chain-libs'
  	project(':react-native-chain-libs').projectDir = new File(rootProject.projectDir, 	'../node_modules/react-native-chain-libs/android')
  	```
3. Insert the following lines inside the dependencies block in `android/app/build.gradle`:
  	```
      compile project(':react-native-chain-libs')
  	```


## Usage
```javascript
import {
    OutputPolicy,
    TransactionBuilder,
    Address,
    Input,
    Value,
    Fee,
    TransactionFinalizer,
    Fragment,
    PrivateKey,
    Witness,
    SpendingCounter,
    Hash,
    Account,
} from 'react-native-chain-libs';
​
​function buffer2hex(buffer) {
​   return Array.prototype.map.call(new Uint8Array(buffer), x => ('00' + x.toString(16)).slice(-2)).join('');
​}
​
​const txbuilder = await TransactionBuilder.new_no_payload();
​
​const accountInputAddress = await Address.from_string(
​   'ca1qh9u0nxmnfg7af8ycuygx57p5xgzmnmgtaeer9xun7hly6mlgt3pj2xk344'
​);
​const account = await Account.from_address(accountInputAddress);
​
​const input = await Input.from_account(account, await Value.from_str('1000'));
​
​account.free();
​
​await txbuilder.add_input(input);
​
​await txbuilder.add_output(
​   await Address.from_string(
​       ​'ca1q5nr5pvt9e5p009strshxndrsx5etcentslp2rwj6csm8sfk24a2w3swacn'
​   ),
​   await Value.from_str('500')
​);
​
​const feeAlgorithm = await Fee.linear_fee(
    ​// constant fee
​   await Value.from_str('20'),
​   // coefficient
​   await Value.from_str('5'),
​   // certificate cost
​   await Value.from_str('0')
​);
​
​const finalizedTx = await txbuilder.seal_with_output_policy(
    ​feeAlgorithm,
    ​​await OutputPolicy.one(accountInputAddress)
​);
​
​feeAlgorithm.free();
​
​const finalizer = await TransactionFinalizer.new(finalizedTx);
​
​// Sign the transaction
​// We need the genesis hash, the transaction id and the input account private key
​const genesisHash =
    ​'6a702a181151b772ca0acbdc4d2870ed80c09b626b29fffc2e47abf2330ad0cd';
​const txid = await finalizer.get_tx_sign_data_hash();
​const privateKey = await PrivateKey.from_bech32(
    ​'ed25519e_sk1gz0ff4w444nwejap5shxrllypz5euswq6wn04fffzes02atw99xkd4jn838v3vrfg9eqt7f4sxjlsy0tdcmj0d2dqvwc8ztwgyfnwyszvjg32'
​);
​
​const witness = await Witness.for_account(
    ​await Hash.from_hex(genesisHash),
    ​txid,
​   privateKey,
​   await SpendingCounter.zero()
​);
​
​await finalizer.set_witness(0, witness);
​
​const signedTx = await finalizer.build();
​
​const readyToSendTx = await Fragment.from_generated_transaction(signedTx);
​
​const readyToSendTxBytes = await readyToSendTx.as_bytes();
​
​readyToSendTx.free();
​
​console.log(buffer2hex(readyToSendTxBytes.buffer));
```
