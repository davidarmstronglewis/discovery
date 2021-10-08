openocd:
	openocd -f interface/stlink.cfg -f target/stm32f3x.cfg

readobj:
	cargo readobj --target thumbv7em-none-eabihf --bin hello -- --file-header

gdb:
	arm-none-eabi-gdb -q -ex "target remote :3333" ../target/thumbv7em-none-eabihf/debug/hello

