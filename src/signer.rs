use alloy::{
    primitives::{Address, PrimitiveSignature},
    signers::local::PrivateKeySigner,
};
use eyre::{Error, Result};

pub struct Signer {
    address: Address,
    signer: PrivateKeySigner,
}

impl Signer {
    pub fn new(private_key: &str) -> Result<Self, Error> {
        let signer: PrivateKeySigner = private_key.parse().expect("Invalid private key");
        let address = signer.address();
        Ok(Self { address, signer })
    }

    pub fn sign(&self) -> Result<PrimitiveSignature, Error> {
        unimplemented!();
    }
}
