use diff;
use openssl::{
    ec::{EcGroup, EcKey},
    envelope::Seal,
    error::ErrorStack,
    hash::{Hasher, MessageDigest},
    nid::Nid,
    pkey::{self, PKey, Private, Public},
    sign::Signer,
    symm::Cipher,
};

/// public and private key pair
pub struct KeyPair {
    public: PKey<Public>,
    private: Option<PKey<Private>>,
}

impl KeyPair {
    /// creates a new keypair using the Elliptic Curve algorithm
    pub fn new() -> Result<Self, ErrorStack> {
        let nid = Nid::X9_62_PRIME256V1;
        let group = EcGroup::from_curve_name(nid)?;
        let key = EcKey::generate(&group)?;

        let priv_key = pkey::PKey::from_ec_key(key.clone())?;
        let pub_key = pkey::PKey::public_key_from_der(&key.public_key_to_der()?)?;

        Ok(KeyPair {
            public: pub_key,
            private: Some(priv_key),
        })
    }
    /// return public key
    pub fn public_key(&self) -> &pkey::PKey<Public> {
        &self.public
    }
    /// return private key
    pub fn private_key(&self) -> Option<&PKey<Private>> {
        match &self.private {
            Some(key) => Some(&key),
            None => None,
        }
    }
}

/// identifier based on a public key
pub type Id = Vec<u8>;

/// makes a hash of a public key
pub fn get_id(pub_key: &PKey<Public>) -> Result<Id, ErrorStack> {
    let mut hasher = Hasher::new(MessageDigest::sha256())?;
    hasher.update(&pub_key.raw_public_key()?)?;
    let digest = hasher.finish()?.to_vec();
    Ok(digest)
}

/// represents what was changed in two versions of a file
#[derive(Debug, PartialEq, Eq)]
pub enum Changes<T> {
    Removed(T),
    Added(T),
}
pub type differences<'a> = Vec<Changes<&'a str>>;
/// function used to return the differences between two files
pub fn changes<'a>(original: &'a str, new: &'a str) -> differences<'a> {
    // let result =
    diff::lines(original, new)
        .iter()
        .filter_map(|s| match s {
            diff::Result::Left(r) => Some(Changes::Removed(*r)),
            diff::Result::Right(a) => Some(Changes::Added(*a)),
            diff::Result::Both(_, _) => None,
        })
        .collect()
}
pub type signature = Vec<u8>;
// TODO: sign
pub fn sign<T>(data: T, key: &PKey<Private>) -> Vec<u8> {
    unimplemented!();
}

// TODO: timestamp
//   timestamp example: let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

// TODO: encript/envelope
// envelope
// pub fn envelope(data: &[u8], key: PKey<Public>) -> Result<Vec<u8>, ErrorStack> {
//     let cipher = Cipher::aes_256_cbc();

//     let mut seal = Seal::new(cipher, &[key])?;

//     let mut encripted = vec![0; data.len() + cipher.block_size()];

//     seal.update(data, &mut encripted)?;
//     seal.finalize(&mut encripted)?;

//     Ok(encripted)
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_changes() {
        let original = "foo\nbar\nbaz\nquux";
        let new = "foo\nbaz\nbar\nquux";

        let changes = changes(original, new);
        for diff in &changes {
            match diff {
                Changes::Added(a) => println!("+{a}"),
                Changes::Removed(r) => println!("-{r}"),
            }
        }
        assert_eq!(
            changes,
            vec![Changes::Removed("bar"), Changes::Added("bar")]
        )
    }
}
