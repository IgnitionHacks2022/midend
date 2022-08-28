<div align="center">

# garbagio_midend

raspberry pi controller software for the garbagio hardware

</div>

controller program for **garbagio** designed to be ran on a raspberry pi.
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

Ensure that you have the `opencv` headers and `clang` installed on your system.
To run
```
$ just
```

## HARDWARE SETUP

this project needs access to a couple of peripherals, notably it needs access
to a webcam thats mounted at `/dev/videoX`, and a speaker.

