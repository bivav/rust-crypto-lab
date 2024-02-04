### Caesar Cipher Implementation

___

**Description:**\
A simple Caesar Cipher program that encrypts and decrypts messages.

**Usage in Rust** \
From the root directory:

```
cargo run --package caesar_cipher
```

Or, from the **caesar_cipher** directory:

```
cargo run
```

___

### Caesar Cipher

The `caesar_cipher` workspace is a Rust project that provides functionality for a Caesar Cipher encryption and
decryption. It contains two main files: `main.rs` and `lib.rs` located in the `src` directory.

The `main.rs` file contains the main function that drives the program. It provides a user interface for the user to
choose between encrypting and decrypting text. The user's choice is read from the standard input. If the user chooses to
encrypt or decrypt, the program gets the text and shift key from the user, creates a new `CaesarCipher` with the text
and key, and then encrypts or decrypts the text accordingly. The result is then printed to the standard output. The
program runs in an infinite loop until the user chooses to quit.

The `lib.rs` file contains the definition of the `CaesarCipher` struct and its associated methods, as well as
the `get_text_key` function. The `CaesarCipher` struct has two fields: `text` and `key`, which represent the text to be
encrypted or decrypted and the shift key, respectively. The `CaesarCipher` struct has two methods: `encrypt`
and `decrypt`. The `encrypt` method shifts the characters in the text by the key value to encrypt it, while
the `decrypt` method shifts the characters in the opposite direction to decrypt it. The `get_text_key` function is used
to get the text and shift key from the user. It reads the text and key from the standard input and returns them as a
tuple.

The `Cargo.toml` file in the root directory of the workspace specifies that `caesar_cipher` is a member of the
workspace. This means that it is part of a larger project that may contain other packages.