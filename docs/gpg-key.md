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

## Exporting the gpg key

run:

	$ gpg --export --armor EMAIL@HERE.COM >gpg.pub
