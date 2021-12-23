use magic_crypt::MagicCryptTrait;

#[allow(dead_code)]
pub fn encrypt(msg: String) -> String {
    let mc = new_magic_crypt!("grassfed_butter", 256);
    mc.encrypt_str_to_base64(msg)
}

pub fn decrypt(msg: String) -> String {
    let mc = new_magic_crypt!("grassfed_butter", 256);
    mc.decrypt_base64_to_string(msg).unwrap()
}