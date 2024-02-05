### Rust Cryptography Lab

**NOTE: This project is not a part of Rust Foundations, but is a personal project for learning and practicing Rust only.**
___
Exploring the Rust programming language and working on some projects related to cryptography.

The project consists of four workspaces: `"caesar_cipher"`, `"vigenere_cipher"`, `"file_encrypt_decrypt_aes"` as of now.

The `"caesar_cipher"` workspace is a Rust project that provides functionality for a Caesar Cipher encryption and
decryption. It contains a user interface for the user to choose between encrypting and decrypting text. The user's
choice is read from the standard input. If the user chooses to encrypt or decrypt, the program gets the text and shift
key from the user, creates a new `CaesarCipher` with the text and key, and then encrypts or decrypts the text
accordingly.

The `"vigenere_cipher"` workspace is a Rust project that provides functionality for a Vigenere Cipher encryption and
decryption. Similar to the Caesar Cipher program, it provides a user interface for the user to choose between encrypting
and decrypting text. The user's choice is read from the standard input. If the user chooses to encrypt or decrypt, the
program gets the text and shift keyword from the user, creates a new `VigenereCipher` with the text and keyword, and
then encrypts or decrypts the text accordingly.

The `"file_encrypt_decrypt_aes"` workspace is a Rust project that provides functionality for file encryption and
decryption using AES (Advanced Encryption Standard) with a 256-bit key and also uses Hash Function Utility (SHA256). It reads
command-line arguments to determine whether to encrypt or decrypt a file. If the command is "encrypt", it reads the
file, generates a hash of the file
content, asks the user for a password, and then encrypts the file content using the password. The encrypted data, along
with the hash, salt, and initialization vector (IV), are then saved to a new file in base64 format.
