use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;
pub fn encrypt_file(filename: &str, key: &[u8], iv: &[u8]) -> Result<(), Box<dyn Error>> {
    let cipher = Aes256Cbc::new_from_slices(key, iv)?;

    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let encrypted_text = cipher.encrypt_vec(&data);

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}.enc", filename))?;

    output.write_all(&iv)?;
    output.write_all(&encrypted_text)?;

    Ok(())
}

pub fn decrypt_file(filename: &str, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut f = std::fs::File::open(filename)?;
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;

    const IV_LEN: usize = 16;
    if data.len() < IV_LEN {
        return Err("encrypted file is too short: missing IV".into());
    }

    let iv = &data[..IV_LEN];
    let encrypted_text = &data[IV_LEN..];

    if encrypted_text.is_empty() {
        return Err("encrypted file contains no ciphertext".into());
    }

    let cipher = Aes256Cbc::new_from_slices(key, iv)?;

    let decrypted_text = cipher.decrypt_vec(encrypted_text)?;

    let out_name = filename
        .strip_suffix(".enc")
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{filename}.dec"));

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(out_name)?;

    output.write_all(&decrypted_text)?;

    Ok(())
}