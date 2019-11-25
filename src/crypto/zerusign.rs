use base64::{ decode, encode, DecodeError };
use secp256k1::{ Secp256k1 };
use sha2::{ Sha256, Digest };
use ripemd160::{ Ripemd160 };
use basex_rs::{ BaseX, Encode, Decode, BITCOIN};
use bitcoin::consensus::encode::{ VarInt, serialize };
use log::*;

#[derive(Debug)]
pub enum Error {
  DecodeSignatureFailure,
  RecoveryIdFailure,
  RecoverableSignatureFailure,
  MessageFailure,
  PublicKeyFailure,
  AddressMismatch(String),
  PrivateKeyFailure,
}

impl From<DecodeError> for Error {
  fn from(_: DecodeError) -> Error {
    Error::DecodeSignatureFailure
  }
}

impl From<secp256k1::Error> for Error {
  fn from(error: secp256k1::Error) -> Error {
    match error {
      secp256k1::Error::InvalidRecoveryId => Error::RecoveryIdFailure,
      secp256k1::Error::InvalidSignature => Error::RecoverableSignatureFailure,
      secp256k1::Error::InvalidMessage => Error::MessageFailure,
      secp256k1::Error::InvalidPublicKey => Error::PublicKeyFailure,
      _ => Error::RecoverableSignatureFailure,
    }
  }
}

pub fn sha256d (input: &[u8]) -> Vec<u8> {
  let mut hasher1 = Sha256::default();
  hasher1.input(input);
  let mut hasher2 = Sha256::default();
  hasher2.input(hasher1.result());
  return hasher2.result().into_iter().collect();
}

pub fn hash160 (input: &[u8]) -> Vec<u8> {
  let mut hasher1 = Sha256::default();
  hasher1.input(input);
  let mut hasher2 = Ripemd160::default();
  hasher2.input(hasher1.result());
  return hasher2.result().into_iter().collect();
}

static MSG_SIGN_PREFIX: &'static [u8] = b"\x18Bitcoin Signed Message:\n";

pub fn msg_hash (msg: &str) -> Vec<u8> {
  let bytes;
  bytes = serialize(&VarInt(msg.len() as u64));
  sha256d(&[
    MSG_SIGN_PREFIX,
    bytes.as_slice(),
    msg.as_bytes()
    ].concat())
}

pub fn verify(data: String, valid_address: String, sign: String) -> Result<(), Error> {
  let sig = decode(&sign)?;
  let hash = msg_hash(&data);

  let (sig_first, sig_r) = match sig.split_first() {
    Some(t) => t,
    None => return Err(Error::DecodeSignatureFailure),
  };

  let rec_id_v = (sig_first - 27) & 3;
  // let rec_compact = (sig_first - 27) & 4 != 0;
  let rec_id = secp256k1::recovery::RecoveryId::from_i32(rec_id_v as i32)?;
  let signature = secp256k1::recovery::RecoverableSignature::from_compact(&sig_r, rec_id)?;
  let message = secp256k1::Message::from_slice(hash.as_slice())?;
  let secp = Secp256k1::new();
  let recovered: secp256k1::PublicKey = secp.recover(&message, &signature)?;
  let serialized = recovered.serialize_uncompressed();

  let hashed = hash160(&serialized);
  let version = [0u8];
  let hashed2 = sha256d(&[&version, hashed.as_slice()].concat());
  let out = &[&version, hashed.as_slice(), hashed2.get(0..4).unwrap()].concat();

  let address = BaseX::new(BITCOIN).encode(out);

  if address == valid_address {
    return Ok(())
  }
  return Err(Error::AddressMismatch(address))
}

pub fn sign (data: String, privkey: String) -> Result<(String), Error> {
  let hex = match BaseX::new(BITCOIN).decode(privkey.clone()) {
    Some(h) => h,
    None => return Err(Error::PrivateKeyFailure),
  };
  let privkey = secp256k1::SecretKey::from_slice(&hex[1..33])?;
  let hash = msg_hash(&data);
  let message = secp256k1::Message::from_slice(hash.as_slice())?;
  let secp = Secp256k1::new();
  let sig = secp.sign_recoverable(&message, &privkey);
  let (rec_id, ser_c) = sig.serialize_compact();
  let ser_c_v: [&[u8]; 2] = [
    &[(rec_id.to_i32() + 27) as u8],
    &ser_c
  ];

  let s = encode(&ser_c_v.concat());
  return Ok(s)
}

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod tests {
  use std::str::FromStr;
  const PUBKEY: &str = "1HZwkjkeaoZfTSaJxDw6aKkxp45agDiEzN";
  const PRIVKEY: &str = "5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss";
  const MESSAGE: &str = "Testmessage";
  const SIGNATURE: &str = "G+Hnv6dXxOAmtCj8MwQrOh5m5bV9QrmQi7DSGKiRGm9TWqWP3c5uYxUI/C/c+m9+LtYO26GbVnvuwu7hVPpUdow=";
  const MSG_HASH: &[u8] = &[250, 76, 36, 63, 188, 246, 57, 82, 210, 190, 131, 30, 80, 21, 194, 116, 202, 29, 102, 133, 20, 205, 34, 11, 215, 177, 255, 148, 166, 130, 107, 161];

  fn get_testdata_verify() -> (String, String, String) {
    (
      String::from_str(MESSAGE).unwrap(),
      String::from_str(PUBKEY).unwrap(),
      String::from_str(SIGNATURE).unwrap(),
    )
  }

  fn get_testdata_sign() -> (String, String, String) {
    (
      String::from_str(MESSAGE).unwrap(),
      String::from_str(PUBKEY).unwrap(),
      String::from_str(PRIVKEY).unwrap(),
    )
  }

  fn get_testdata_msg_hash() -> (String, &'static[u8]) {
    (
      String::from_str(MESSAGE).unwrap(),
      MSG_HASH,
    )
  }

  #[test]
  fn test_msg_hash() {
    let (msg, hash) = get_testdata_msg_hash();
    let result = super::msg_hash(&msg);
    assert_eq!(result, hash);
  }

  #[test]
  fn test_verification() {
    let (msg, key, sig) = get_testdata_verify();
    let result = super::verify(msg, key, sig);
    assert_eq!(result.is_ok(), true)
  }

  #[test]
  fn test_signing() {
    let (msg, pubkey, privkey) = get_testdata_sign();
    let result = super::sign(msg.clone(), privkey);
    assert_eq!(result.is_ok(), true);
    let result2 = super::verify(msg, pubkey, result.unwrap());
    assert_eq!(result2.is_ok(), true);
  }
}
