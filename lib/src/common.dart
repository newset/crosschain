// ignore_for_file: constant_identifier_names

abstract class Client {
  Map config = {};
}

final networks = [{}];

enum Chain { BitCoin, Etherum, Tron, Ton }

class Account {
  late Client client;
}

class BitCoin extends Client {}

class Etherum extends Client {}

class Tron extends Client {}

class Ton extends Client {}
