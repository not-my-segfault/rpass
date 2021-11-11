# passman
Simple Rust-based password generator based on qwertycards

## how it works
passman is simple in how it generates a password. Each password has 4 components:
- the prefix
- the secret
- the app name
- the cipher

when you run passman for the first time, it generates a prefix (random 7 character alphanumeric sequence), a cipher (random 26 character alphanumeric sequence) and prompts you for a secret (memorable phrase of your choice). it then stores those as files in ~/.

then, for every password you want to generate, it asks you for the name of the service (e.g., google, amazon, etc.)

it then takes your app name and maps it against the cipher it generated earlier.

the result of this is a secure, pseudo-randomized, memorable and most importantly *reproducible* password.

## todo
- encrypt the files with a master password
