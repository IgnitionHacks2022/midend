<div align="center">

# indabin_midend

raspberry pi controller software for the indabin hardware

</div>

controller program for **indabin** designed to be ran on a raspberry pi.
handles bluetooth proximity, image capture of the waste as well as control of
the motor.

this repo contains two binaries:
- `server`: the main server binary. handles communicating with the backend, and
  interfacing with the hardware
- `gpio-debug`: a simple repl for interfacing with the hardware

## DEV SETUP

Some features of the nightly build of `rustfmt` are required. Ensure that you
have ran:
```
$ rustup install nightly
```

Next install some git hooks:
```
$ just devsetup
```

### CROSS COMPILATION (WIP)

to be able to compile to the raspberry pi (4), ensure that you have the correct
rust toolchain installed:
```
$ rustup target add armv7-unknown-linux-gnueabihf
```

to be able to compile to arm from an x86 machine, we need to install a
cross-compile toolchain (this
[blog](https://shaneutt.com/blog/rust-x86-arm-raspberry-pi-4/) is a pretty nice
guide). on arch based distros this can be done by (adapt based on your system):
```
$ yay -S arm-linux-gnueabihf-gcc
```

Ensure that you have the `opencv` headers, `libasound2` and `clang` installed
on your system.

## HARDWARE SETUP

this project needs access to a couple of peripherals, notably it needs access
to a webcam thats mounted at `/dev/videoX`, and a speaker.

