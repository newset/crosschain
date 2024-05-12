import 'package:crosschain/crosschain.dart';
import 'package:crosschain/src/common.dart';
import 'package:crosschain/src/mnemonic.dart';
import 'package:test/test.dart';

void main() {
  group('A group of tests', () {
    final awesome = Awesome();

    setUp(() {
      // Additional setup goes here.
    });

    test('First Test', () {
      generate_nemonic(Chain.Ton);

      generate_nemonic(Chain.Etherum);
      expect(awesome.isAwesome, isTrue);
    });
  });
}
