import 'package:decimal/decimal.dart';

/// 小数转BigInt
BigInt toBigAmount(String amount, [int decimals = 18]) {
  final d = Decimal.parse(amount);

  if (d.scale > decimals) {
    throw '';
  }
  final mod = BigInt.from(10).pow(decimals);
  return (d * Decimal.fromBigInt(mod)).toBigInt();
}

/// BigInt转小数
double parseAmount(BigInt amout, [int decimals = 18]) {
  final mod = BigInt.from(10).pow(decimals);
  final d = Decimal.fromBigInt(amout) / Decimal.fromBigInt(mod);
  return d.toDouble();
}

