# Configuring a Linux server with SSH Public Key Authentication

## Create the ~/.ssh folder if it does not already exists:

	$ mkdir ~/.ssh
	$ chmod 700 ~/.ssh

## Creating the SSH Public and Private Key inside the ssh folder:

	$ cd ~/.ssh
	$ ssh-keygen -t rsa -C "Your Name <email@here.com>"

## Authorize any ssh client with the public key to log in:

	$ cd ~/.ssh
	$ cat id_rsa.pub >> ~/.ssh/authorized_keys
	$ chmod 600 ~/.ssh/authorized_keys

## Disable password Authetication

	# cd /etc/ssh
	# cp sshd_config sshd_config.orig
	# vim sshd_config

change this:

	PermitRootLogin			yes
	PasswordAuthentication	yes
	UsePAM					yes

to this:

	PermitRootLogin			no
	PasswordAuthentication	no
	UsePAM					no

save it and then restart ssh service with:

	$ sudo systemctl restart sshd
