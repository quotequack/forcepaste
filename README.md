# FORCE PASTE

![image](https://github.com/user-attachments/assets/e4d2c802-074f-4e0c-9a7e-5c3721df10da)


**Forcepaste** is a simple tool to force paste text into a textbox even if it does not allow pasting.
## Usage
Just open the program and paste the text you want to the force paste input box and click "Paste". After 2 seconds the text will be pasted by simulating keyboard input.
## Why?
I made this because I was annoyed that I could not paste text into a textbox that did not allow pasting. This is a simple and reliable solution to this problem.
## Compatability
The program has only been tested in wayland linux but since it uses enigo it most probably works in other environments as well.q
## Installation
Dependencies: Cargo and rust.
```
git clone https://github.com/quotequack/forcepaste
cd forcepaste
cargo build --release
```
#### TO BE ABLE TO RUN THE PROGRAM FROM ANYWHERE:
##### Linux
```
cd target/release && sudo cp forcepaste /usr/bin/
```
##### Windows
If u compiled it on windows u should get a .exe: pin it to your taskbar
