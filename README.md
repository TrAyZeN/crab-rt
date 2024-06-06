<h1 align="center">
    crab-rt
</h1>

> A toy raytracer written in Rust following
[The Ray Tracing in One Weekend series of books](https://raytracing.github.io/).

## Usage
```sh
cargo run --release
```

### Run examples
```sh
cargo run --release --example rt_weekend
cargo run --release --example rt_nextweek
```

## UEFI
crab-rt features a UEFI port in `src/bin/uefi.rs`. To build it run the following
command:
```sh
cargo build --target x86_64-unknown-uefi --no-default-features --features=uefi --release --bin uefi
```

To run it under QEMU:
```sh
mkdir -p esp/efi/boot
cp target/x86_64-unknown-uefi/release/uefi.efi esp/efi/boot/bootx64.efi

cp /usr/share/ovmf/x64/OVMF_CODE.fd .
cp /usr/share/ovmf/x64/OVMF_VARS.fd .

qemu-system-x86_64 --enable-kvm \
    -nodefaults \
    -device virtio-rng-pci \
    -machine q35 \
    -smp 4 \
    -m 256M \
    -vga std \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
```

## Image gallery
<div align="center">
    <img src="assets/scene.png">
    <img src="assets/rt_weekend.png">
    <img src="assets/light.png">
    <img src="assets/cornell_box.png">
</div>
