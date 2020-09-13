# Using MicroPython on the ESP32

MicroPython is a full Python compiler and runtime that runs on the micro-controller hardware. The user is presented with an interactive prompt to execute supported commands immediately. Included are a selection of core Python libraries; MicroPython includes modules which give the programmer access to low-level hardware.

## Sections
- [Requirements](#requirements)
- [Installing the firmware](#installing-the-firmware)
- [Testing the installation](#testing-the-installation)
- [Sending a file](#sending-a-file)
- [Testing a blink](#testing-a-blink)
- [Your best friend](#your-best-friend)

---

## Requirements

- Unix System
- Program for accessing ESP32 serial port (Ex:. picocom)
- Esptool for flashing the board
- Adrafruit Ampy for sending the program files

If you running a Ubuntu-based distro do:

    $ sudo apt install picocom python3-pip
    $ pip3 install --user esptool
    $ pip3 install --user adafruit-ampy

---

## Installing the firmware

First download the micropython firmware for the ESP32 at [this link](https://micropython.org/download/esp32/)

Connect the ESP32 board to your PC and look for it at `/dev/`

    # ls -l /dev/tty*

Usually it's called `ttyUSB0`. So we will asume this port name for the following commands.

Open up a terminal and go to the folder containing the firmware file. First lets erase the flash and then install the firmware:

    $ esptool.py --chip esp32 --port /dev/ttyUSB0 erase_flash

If it's taking some time at the "trying to connect" message you will probably need to hold down the BOOT Button on the ESP32 while trying to connect and it should work. If still not working try to search online for "ESP32 boot mode"

Now let's install the firmware:

    $ esptool.py --chip esp32 --port /dev/ttyUSB0 --baud 460800 write_flash -z 0x1000 firmware_file_name_here.bin

---

## Testing the installation

Let's try connect to the ESP32 using picocom. Run:

    $ picocom -b 115200 /dev/ttyUSBO

First, some picocom instructions: CTRL A + CTRL X closes the connection.

Hit the Reset Button in the ESP32 and you should see some boot messages and you should have a `>>>` witch is the Python Interpreter, you can go ahead and have some fun.

---

## Sending a file

MicroPython and boot up it run 2 files named `boot.py` and `main.py`. Usually we code a `main.py` file the we send it to the ESP32 to test it. You do this by running:

    $ ampy -p /dev/ttyUSB0 -b 115200 put main.py

You can do other things also with ampy. If you curious check it out at [this link](https://github.com/scientifichackers/ampy)

Open a connection again using picocom, hit the Reset Button and you should see some print if the main file does any

---

## Testing a blink

Let's build a simple blink program to check if everything is alright. Create a `main.py` file and paste this:

```python
from machine import Pin
from utime import sleep_ms

LED = Pin(15, Pin.OUT)

while True:
    LED.value(1) # High
    sleep_ms(500)
    LED.value(0) # Low
```

Check the [ESP32 GPIO Pin Out](https://circuits4you.com/wp-content/uploads/2018/12/ESP32-Pinout.jpg) to choose witch port you and for your Led. In this example we use GPIO 15.

Save it and run:

    $ ampy -p /dev/ttyUSB0 -b 115200 put main.py

Reset the board and it should be blinking

## Your best friend

When you get confortable with programming the ESP32, The Doc page will be your best friend to help you at your projects. Check it out at [this link](http://docs.micropython.org/en/latest/).