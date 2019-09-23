use base64::decode;
use secp256k1::{ Secp256k1, recovery };
use sha2::{ Sha256, Digest };
use signatory::generic_array::GenericArray;
use std::str::FromStr;
use ripemd160::{ Ripemd160 };
use basex_rs::{ BaseX, Encode, BITCOIN};
use varmint::WriteVarInt;
use bitcoin::consensus::encode::{ VarInt, serialize };

#[derive(Debug)]
pub enum Error {
  DecodeSignatureFailure,
  RecoveryIdFailure,
  RecoverableSignatureFailure,
  MessageFailure,
  PublicKeyFailure,
  AddressMismatch(String),
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

pub fn msg_hash (msg: String) -> Vec<u8> {
  let mut bytes = vec![];
  // println!("{:X?}", );
  println!("{}", msg.len());
  bytes.write_usize_varint(msg.len()).unwrap();
  // bytes = zero_len(msg.len() as u64);
  bytes = serialize(&VarInt(msg.len() as u64));
  println!("{:X?}", bytes);
  sha256d(&[
    MSG_SIGN_PREFIX,
    bytes.as_slice(),
    msg.as_bytes()
    ].concat())
}

pub fn verify(data: String, valid_address: String, sign: String) -> Result<(),Error> {
  let sig = match decode(&sign) {
    Ok(s) => s,
    Err(_) => return Err(Error::DecodeSignatureFailure),
  };
  println!("msg {}", &data);
  let hash = msg_hash(data);
  println!("hash {:X?}", hash);

  let (sig_first, sig_r) = match sig.split_first() {
    Some(t) => t,
    None => return Err(Error::DecodeSignatureFailure),
  };

  let rec_id_v = (sig_first - 27) & 3;
  let rec_compact = (sig_first - 27) & 4 != 0;
  let rec_id = match secp256k1::recovery::RecoveryId::from_i32(rec_id_v as i32) {
    Ok(r) => r,
    Err(_) => return Err(Error::RecoveryIdFailure),
  };

  let signature = match secp256k1::recovery::RecoverableSignature
    ::from_compact(&sig_r, rec_id) {
      Ok(pk) => pk,
      Err(_) => return Err(Error::RecoverableSignatureFailure),
  };

  let message = match secp256k1::Message::from_slice(hash.as_slice()) {
    Ok(m) => m,
    Err(_) => return Err(Error::MessageFailure),
  };

  let secp = Secp256k1::new();
  let recovered: secp256k1::PublicKey = match secp.recover(&message, &signature) {
    Ok(r) => r,
    Err(_) => return Err(Error::PublicKeyFailure),
  };

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
