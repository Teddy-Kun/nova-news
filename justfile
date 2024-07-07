build:
    cargo b -r

build-win:
    cargo b -r --no-default-features --features winit --target x86_64-pc-windows-gnu