import 'package:crosschain/src/utils.dart';
import 'package:test/test.dart';

void main() {
  group('amount tests', () {
    // final b = BigInt.parse('0.2');

    test('to bigint', () {
      final m = toBigAmount('0.2340', 5);

      expect(m, BigInt.from(23400));
    });

    test("parse bigint", () {
      final p = parseAmount(BigInt.from(23400), 5);
      expect(p.toString(), '0.234');
    });
  });
}

