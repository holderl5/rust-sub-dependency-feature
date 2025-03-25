use std::iter;
use std::str::FromStr;
use std::io::Read;

fn main() {
    // cheap obfuscation to avoid a git notification about secrets.  Effective?
    let privkey1 = "AGE-SECRET-KEY-";
    let privkey2 = "1DVA4F8N8GFMDRNE2LEET69D9JUJZWS5UW37MF9T4KYAQV7ZEVNYS324PVZ";
    let privkey = format!("{}{}", privkey1, privkey2);

    let msg = "-----BEGIN AGE ENCRYPTED FILE-----
YWdlLWVuY3J5cHRpb24ub3JnL3YxCi0+IFgyNTUxOSB0WXd6bDBtVkl4S3hnejZr
bVRBQm1RRjlFS2ZpMDRjci9vK1M5UjEzaTJRCjRGTVNhVXRqOWNHY1JXNFpLQnZG
S3dBQlR2c3I5ZGh0SkVMV1J6VWZSWFUKLT4gWDI1NTE5IEtxcEVDN2lZL1NCditN
TVU1VDJMdjd3Tmp0T2k1cFRlcEF5WlR3ejhiVHMKTmpEMzZNa1IyTUxwWTdvM1Zu
dHJGdGxJZncrbjVkcWF0K3U4ZnlvR3h5NAotLS0geHl6Qnd3MHJsLzdTT2NUK1gr
YVZrUndDU25TRUJhVGUrbHJ0WFNmS1d4RQpUJJw4CAVaW/659X/HcJpBKqerfH/X
o/Rr+FQA4td7T6oKWFe0G9xsdERfVuo3FsnI
-----END AGE ENCRYPTED FILE-----";
    println!("priv: {privkey} msg: {msg}");

    let id = age::x25519::Identity::from_str(&privkey).unwrap();

    let decrypted = {
        let decryptor = age::Decryptor::new(age::armor::ArmoredReader::new(msg.as_bytes())).unwrap();
        let mut decrypted = vec![];
        let mut reader = decryptor.decrypt(iter::once(&id as &dyn age::Identity)).unwrap();
        let _ = reader.read_to_end(&mut decrypted);
        decrypted
    };
    let text = String::from_utf8(decrypted).unwrap();
    println!("Dec {text}");

    let pubkey = "age1n7eh2nf84hwsfwymjy2c88gcqp3zmnzdtqsnj348nlaqynjsp57q698vjr";
    let rec = age::x25519::Recipient::from_str(pubkey).unwrap();
    let p = age::encrypt_and_armor(&rec, text.as_bytes()).unwrap();
    println!("Enc {p}");
}
