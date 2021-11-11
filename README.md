# passman
Simple Rust-based password generator based on qwertycards

## How it works
Passman is simple in how it generates a password. Each password has 4 components:
- The Prefix
- The Secret
- The App Name
- The Cipher

When you run passman for the first time, it generates a prefix (random 7 character alphanumeric sequence), a cipher (random 26 character alphanumeric sequence) and prompts you for a secret (memorable phrase of your choice). It then stores those as files in ~/.

Then, for every password you want to generate, it asks you for the name of the service (e.g., google, amazon, etc.)

It then takes your app name and maps it against the Cipher it generated earlier.

The result of this is a secure, pseudo-randomized, memorable and most importantly *reproducible* password.

## TODO
- Encrypt the files with a master password
