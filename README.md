# Rust binary for randomly crypt key generation

This is an open-source tool for [Crypsetup LUKS](https://gitlab.com/cryptsetup/cryptsetup "Cryptsetup"), under the [luksFormat](https://man.archlinux.org/man/cryptsetup-luksFormat.8.en "Crypsetup luksFormat Man") usage, to generate [OpenSSL](https://docs.openssl.org/master/ "OpenSSL Doc") encrypted key files randomly.

## How to use it

_[**Rust**](https://www.rust-lang.org/ "Rust Lang") and [**OpenSSL**](https://github.com/openssl/openssl "OpenSSL") are required to compile and execute the script._

Basically, the [init.zsh](./init.zsh) script generates the environment variables and compile the binary. **The binary execution will always generate the same key file**. So, to get a new encrypted key it's necessary execute the zsh script again.

This behaviour is thinked to **store the binary** and [create the key on the fly](https://wiki.archlinux.org/title/Mkinitcpio "ArchLinux Doc"), in the [OS boot process](https://wiki.archlinux.org/title/Arch_boot_process "ArchLinux Doc").

## Credits

[ArchLinux Disk Encryption Documentation](https://wiki.archlinux.org/title/Dm-crypt "ArchLinux Doc")

[ArchLinux Disk Encryption Use Cases](https://wiki.archlinux.org/title/Dm-crypt/Specialties#Using_GPG,_LUKS,_or_OpenSSL_encrypted_keyfiles "ArchLinux Doc")