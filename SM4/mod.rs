pub mod cipher;
pub mod cipher_mode;
pub mod error;

pub type Mode = self::cipher_mode::CipherMode;
pub type Cipher = self::cipher_mode::Sm4CipherMode;
