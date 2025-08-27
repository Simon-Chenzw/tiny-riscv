build:
	# TODO build ihex and write hex

debug:
	cargo objdump -- --disassembler-color=on -d

release:
	cargo objdump --release -- --disassembler-color=on -d

.PHONY: build debug release
