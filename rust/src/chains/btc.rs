use std::str::FromStr;

use anychain_bitcoin::{BitcoinAddress, BitcoinFormat, BitcoinTestnet as Testnet};
use anychain_core::Address;
use anychain_kms::bip32::{Prefix, XPrv};
use bip39::{Language, Mnemonic, MnemonicType, Seed};

// 生成mnemonic
pub fn create_account() -> String {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    mnemonic
}

/**
 * P2PKH "m/44'/0'/0'/0/0"
 * P2SH_P2WPKH "m/49'/0'/0'/0/0"
 */
pub fn recover(phrase: String) -> String {
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

    n_add.to_string()
}

pub fn recover_pk(text: String) -> String {
    let nprv = XPrv::from_str(&text).unwrap();

    let n_add =
        BitcoinAddress::<Bitcoin>::from_secret_key(nprv.private_key(), &BitcoinFormat::P2PKH)
            .unwrap();
    n_add.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_account() {
        let (m, _) = create_account();
        println!("{}", m.to_string());
        // assert_eq!(2 + 2, 4);
        assert_eq!(m.to_string().is_empty(), false);

        let a = get_address();
    }
}
