use alloy::{
    primitives::{Address, PrimitiveSignature},
    signers::local::PrivateKeySigner,
};
use eyre::{Context, Error, Result};

pub struct Signer {
    address: Address,
    signer: PrivateKeySigner,
}

impl Signer {
    pub fn new(private_key: &str) -> Result<Self> {
        let signer: PrivateKeySigner = private_key.parse().wrap_err("Invalid private key")?;
        let address = signer.address();
        Ok(Self { address, signer })
    }

    pub fn sign(&self) -> Result<PrimitiveSignature, Error> {
        unimplemented!();
    }
}
