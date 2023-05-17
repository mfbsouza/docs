# Setting up a personal GPG Key

## Creating the gpg key:

to create the key run:

	$ gpg --full-generate-key

Follow this options:

- RSA and RSA (Default)
- 4096
- 0 (does not expire) confirm it
- Real Name
- Email
- empty comment
- confirm it with 'o'
- enter a passphrase

key will be created at $HOME/.gnupg

## Show keys in the machine:

run:

	$ gpg --list-keys

## Backing up the gpg key

for the public key run:

	$ gpg --export --export-options backup --output public.gpg

for the private key run:

    $ gpg --export-secret-keys --export-options backup --output private.gpg

for the trust relationships run:

    $ gpg --export-ownertrust > trust.gpg

## Restoring keys

    $ gpg --import public.gpg
    $ gpg --import private.gpg
    $ gpg --import-ownertrust trust.gpg
