use std::fs;

use anychain_bitcoin::{address::BitcoinAddress, Bitcoin, BitcoinFormat, BitcoinTestnet};

use anychain_core::Address;
use anychain_kms::{
    bip32::{DerivationPath, Prefix, XPrv},
    bip39::{Language, Mnemonic, Seed},
};

fn main() {
    test_pk()
}

fn get_md() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}

fn test_pk() {
    let phrase: &str = "heavy face learn track claw jaguar pigeon uncle seven enough glow where";
    // let mnemonic =  Mnemonic::new(MnemonicType::Words12,Language::English);
    let mnemonic: Mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    //let phrase = mnemonic.phrase();
    let seed: Seed = Seed::new(&mnemonic, "");
    // let path: DerivationPath = "m".parse().unwrap();
    let npath: DerivationPath = "m/44'/0'/0'/0/0".parse().unwrap();

    let nprv = XPrv::new_from_path(&seed, &npath).unwrap();
    // address::BitcoinAddress
    let n_add =
        BitcoinAddress::<Bitcoin>::from_secret_key(nprv.private_key(), &BitcoinFormat::P2PKH)
            .unwrap();

    let path: DerivationPath = "m/49'/0'/0'/0/0".parse().unwrap();
    let wprv = XPrv::new_from_path(&seed, &path).unwrap();

    let w_add =
        BitcoinAddress::<Bitcoin>::from_secret_key(wprv.private_key(), &BitcoinFormat::P2SH_P2WPKH)
            .unwrap();

    //let ek = xprv.to_extended_key(Prefix::XPRV);
    // println!("xprv:{:?}", xprv.to_string(Prefix::XPRV));
    println!("addr:{:}", n_add.to_string());
    println!("tsetAdd:{:}", w_add.to_string());
}
