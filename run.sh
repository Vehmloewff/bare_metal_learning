qemu-system-x86_64 \
    -drive format=raw,file=target/agincourt.img \
    -m 64M \
    -nographic \
    -serial mon:stdio
