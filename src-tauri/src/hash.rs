// argon2?
use blake2::{Blake2b512, Blake2s256};
use digest::Digest;
use hex::encode_upper;
use md5::Md5;
use ripemd::{Ripemd160, Ripemd320};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
// use blake3;
use fsb::{Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};
use gost94::Gost94CryptoPro;
use groestl::Groestl256;
use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};
use sm3::Sm3;
use streebog::*;
use tiger::Tiger;
use whirlpool::Whirlpool;

use crate::event::emit_event;
use crate::file::{Path, FileCacher, read_file};

pub type Algorithm = String;
pub type Hex = String;

#[derive(Clone, serde::Serialize)]
pub struct FileHash {
  path: String,
  hashes: Vec<Hash>
}

#[derive(Clone, serde::Serialize)]
pub struct Hash {
  algo: String,
  hash: String
}

#[tauri::command(async)]
pub fn hash_files(path: String, algos: Vec<Algorithm>) -> FileHash {
  let file_hash: FileHash;
  // for path in &paths {
    emit_event("hash", &path);
    let mut hashes: Vec<Hash> = Vec::new();
    for algo in &algos {
      let hash = hash_file(&path, algo);
      hashes.push(Hash{
          algo: algo.to_string(), 
          hash: hash.to_string()
        });
    }
    file_hash = FileHash{
      path: path.to_string(), 
      hashes
    };
  // }
  return file_hash;
}

pub fn hash_file(path: &Path, algo: &str) -> Hex {
  match algo {
    "MD5" => return hasher(path, Md5::default()),
    "SHA1" => return hasher(path, Sha1::default()),
    "SHA256" => return hasher(path, Sha256::default()),
    "SHA384" => return hasher(path, Sha384::default()),
    "SHA512" => return hasher(path, Sha512::default()),
    "SHA3-224" => return hasher(path, Sha3_224::new()),
    "SHA3-256" => return hasher(path, Sha3_256::new()),
    "SHA3-384" => return hasher(path, Sha3_384::new()),
    "SHA3-512" => return hasher(path, Sha3_512::new()),
    "RIPEMD160" => return hasher(path, Ripemd160::default()),
    "RIPEMD320" => return hasher(path, Ripemd320::default()),
    "BLAKE2S" => return hasher(path, Blake2s256::default()),
    "BLAKE2B" => return hasher(path, Blake2b512::default()),
    "WHIRLPOOL" => return hasher(path, Whirlpool::default()),
    "SHABAL192" => return hasher(path, Shabal192::new()),
    "SHABAL224" => return hasher(path, Shabal224::new()),
    "SHABAL256" => return hasher(path, Shabal256::new()),
    "SHABAL384" => return hasher(path, Shabal384::new()),
    "SHABAL512" => return hasher(path, Shabal512::new()),
    "STREEBOG256" => return hasher(path, Streebog256::new()),
    "STREEBOG512" => return hasher(path, Streebog512::new()),
    "TIGER" => return hasher(path, Tiger::default()),
    "SM3" => return hasher(path, Sm3::default()),
    "GROESTL" => return hasher(path, Groestl256::default()),
    "GOST" => return hasher(path, Gost94CryptoPro::default()),
    "FSB-160" => return hasher(path, Fsb160::default()),
    "FSB-224" => return hasher(path, Fsb224::default()),
    "FSB-256" => return hasher(path, Fsb256::default()),
    "FSB-384" => return hasher(path, Fsb384::default()),
    "FSB-512" => return hasher(path, Fsb512::default()),
    _ => return hasher(path, Sha256::default()),
  }
}

use std::io;

use std::time::Instant;

fn hasher<T>(path: &Path, mut hasher: T) -> Hex
where
  T: Digest,
  T: io::Write,
{

  let mut read = FileCacher::new(|path| {
    let bytes = read_file(path);
    bytes
  });

  let now = Instant::now();
  let bytes = read.value(&path);
  hasher.update(bytes);
  let elapsed_time = now.elapsed();
  println!("path: {} took {} ms.", path, elapsed_time.as_millis());
  return encode_upper(hasher.finalize());
}

// old hash caching code.

// pub fn get_hash(path: &Path, algo: &Algorithm) -> Hex {
//   let mut cache = HASH_CACHE.lock().unwrap();
//   let hashes = cache.get_mut(path);

//   match hashes {
//     Some(hashes) => {
//       // existing path w/ some hashes.
//       let hash = hashes.get(algo);

//       match hash {
//         // we have the file cached.
//         Some(hash) => {
//           emit_event("hash", &[algo, "Found in cache!"].join("-"));
//           hash.to_string()
//         }

//         // we don't have the file cached.
//         None => {
//           emit_event("hash", &[algo, "Not found in cache."].join("-"));
//           let hash = hash_file(path, algo);
//           hashes.insert(algo.to_string(), hash.clone());
//           drop(cache);
//           hash
//         }
//       }
//     }
//     // new file.
//     None => {
//       emit_event("hash", &["No cache found."].join(" "));
//       let hash = hash_file(path, algo);
//       let hash_map: HashMap<Algorithm, Hex> = HashMap::from([(algo.to_string(), hash.clone())]);
//       cache.insert(path.to_string(), hash_map);
//       drop(cache);
//       hash
//     }
//   }
// }