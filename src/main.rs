mod aio; // Input/Output system actions
mod cmd; // OS command execution
mod cnv; // String's conversions
mod env; // Binary execution's environment constants

fn main() {
    // INFO: Get the environment's variables
    let constants: Vec<&str> = env::constants();

    // INFO: Decrypted hashed key
    let hash_filename: String = String::from("./hash.tmp");
    let mut hash_key: String = String::new();
    for args_order in constants[0].split(',').collect::<Vec<&str>>() {
        let mut key: String = String::new();
        args_order.chars().for_each(|index| {
            key += constants[usize::from_str_radix(&String::from(index), 10).unwrap()]
        });
        aio::write_file(&hash_filename, &(hash_key.clone() + &key).as_bytes());
        hash_key = hash_key
            + &cnv::str_rsplit_delimiter(
                cnv::utf8_to_str(cmd::pipe(
                    "cat",
                    vec![&hash_filename],
                    "openssl",
                    vec!["dgst", "-sha512"],
                )),
                "=",
            );
        cnv::utf8_to_str(cmd::output("rm", vec!["-f", &hash_filename]));
    }

    aio::write_file(&hash_filename, hash_key.as_bytes());

    // INFO: Encrypted key
    let crypt_key_filename = String::from("./crypt.key");

    cmd::output(
        "openssl",
        vec![
            "enc",
            "-aes-256-ctr",
            "-pbkdf2",
            "-saltlen",
            "16",
            "-S",
            "",
            "-pass",
            "pass:",
            "-in",
            &hash_filename,
            "-out",
            &crypt_key_filename,
        ],
    );

    cnv::utf8_to_str(cmd::output("rm", vec!["-f", &hash_filename]));
}
