# azure-iot-rs-sys
Rust bindings for azure_iot_sdk_c


## Building 
### Clone the minimum needed submodules.
```
git submodule update --init --depth 1
cd azure-iot-sdk-c
git submodule update --init --depth 1 umqtt/
git submodule update --init --depth 1 uamqp/
git submodule update --init --depth 1 c-utility/
git submodule update --init --depth 1 deps/uhttp/
git submodule update --init --depth 1 deps/umock-c/
git submodule update --init --depth 1 deps/parson/
git submodule update --init --depth 1 deps/azure-macrtoutils-c/

```
### Install the dependencies.
```
sudo apt-get install -y git cmake build-essential curl libcurl4-openssl-dev libssl-dev uuid-dev
```
