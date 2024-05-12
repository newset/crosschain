import 'package:crosschain/src/common.dart';
import 'package:bip39/bip39.dart' as bip39;
import 'package:flutter_bitcoin/flutter_bitcoin.dart' show HDWallet;

String generate_nemonic(Chain chain) {
  late String code;
  switch (chain) {
    case Chain.Ton:
      code = bip39.generateMnemonic(strength: 256);
    default:
      code = bip39.generateMnemonic(strength: 128);
  }
  print('data: $chain $code');
  return code;
}

address(String mnemonic) {
  final seed = bip39.mnemonicToSeed(mnemonic);
  final hdWallet = HDWallet.fromSeed(seed);
  // btcWallet = hdWallet.derivePath("m/44'/0'/0'/0/0");
  final ethWallet = hdWallet.derivePath("m/44'/60'/0'/0/0");
  // tronWallet = hdWallet.derivePath("m/44'/195'/0'/0/0");
  // return ethWallet.address;
}
