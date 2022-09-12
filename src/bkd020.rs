use crate::BdkDetails;
use bdk20::bitcoin::secp256k1::Secp256k1;
use bdk20::bitcoin::util::bip32::ExtendedPrivKey;
use bdk20::bitcoin::Network;
use bdk20::blockchain::ElectrumBlockchain;
use bdk20::template::Bip84;
use bdk20::wallet::{wallet_name_from_descriptor, AddressIndex, SyncOptions};
use bdk20::KeychainKind;
use bdk20::Wallet;
use bdk20::{electrum_client, sled};
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) fn execute(xprv: &str) -> Result<BdkDetails, Box<dyn Error>> {
    let xprv = ExtendedPrivKey::from_str(xprv).unwrap();

    let wallet_name = wallet_name_from_descriptor(
        Bip84(xprv.clone(), KeychainKind::External),
        Some(Bip84(xprv.clone(), KeychainKind::Internal)),
        Network::Regtest,
        &Secp256k1::new(),
    )?;

    let mut datadir = PathBuf::from_str("./")?;
    datadir.push("wallet-example");
    let database = sled::open(datadir)?;
    let database = database.open_tree(wallet_name.clone())?;

    let client = electrum_client::Client::new("ssl://blockstream.info:993").unwrap();

    let wallet = Wallet::new(
        Bip84(xprv.clone(), KeychainKind::External),
        Some(Bip84(xprv.clone(), KeychainKind::Internal)),
        Network::Testnet,
        database,
    )?;

    let index_0 = wallet.get_address(AddressIndex::Peek(0)).unwrap();
    let index_1 = wallet.get_address(AddressIndex::Peek(1)).unwrap();

    let blockchain = ElectrumBlockchain::from(client);
    wallet.sync(&blockchain, SyncOptions::default())?;

    let last_unused = wallet.get_address(AddressIndex::LastUnused).unwrap();

    Ok(BdkDetails {
        name: wallet_name,
        address_index_0: index_0.address.to_string(),
        address_index_1: index_1.address.to_string(),
        last_unused: last_unused.address.to_string(),
        balance: wallet.get_balance().unwrap(),
    })
}
