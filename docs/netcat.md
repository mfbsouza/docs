# Using netcat

netcat is a computer networking utility for reading from and writing to network
connections using TCP or UDP

## Launching netcat

Using netcat to listen at port 4040:

	$ nc -l 4040

or in verbose mode:

    $ nc -v -l 4040

Using netcat as a client:

    $ nc <ip> <port>

or in verbose mode and without killing the server on closing the connection:

    $ nc -v -k <ip> <port>

## Sending files

on the server:

    $ nc -v -l 4040 > output

on the client:

    $ nc -w 2 <ip> <port> < input
