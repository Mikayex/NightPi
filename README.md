# NightPi
Control software written in Rust for my DIY camera trap based on Raspberry Pi Zero W.

## Prerequisite
You need [cargo-make](https://github.com/sagiegurari/cargo-make) (`cargo install cargo-make`) as well as Docker to build this program.

## Building
For debug build: `cargo make build`

For release build: `cargo make build-release`

## Deploying/Running on device through SSH
Set theses environment variables:
* `NIGHTPI_DEPLOY_IP` to the user@ip of the device (for example `pi@192.168.0.30`)
* `NIGHTPI_DEPLOY_DIR` to the target directory on device to copy the executable to (for example `/home/pi`)

### Deploy only
For debug build: `cargo make deploy`

For release build: `cargo make deploy-release`

### Deploy and run
For debug build: `cargo make run`

For release build: `cargo make run-release`
