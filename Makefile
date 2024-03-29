all: Disk.img

00.BootLoader/BootLoader.bin:
	make -C 00.BootLoader

01.Kernel32/Kernel32.bin:
	make -C 01.Kernel32

image_maker:
	cd 04.Utility && cd 00.ImageMaker && cargo build --release
	cp 04.Utility/00.ImageMaker/target/release/image_maker .

Disk.img: image_maker 00.BootLoader/BootLoader.bin 01.Kernel32/Kernel32.bin
	./image_maker 00.BootLoader/BootLoader.bin 01.Kernel32/Kernel32.bin

clean:
	make -C 00.BootLoader clean
	make -C 01.Kernel32 clean
	rm -f Disk.img
	rm -f image_maker

test: clean
	make
	qemu-system-x86_64 \
    -L . -m 64 -M pc \
    -blockdev driver=file,node-name=f0,filename=./Disk.img \
    -device floppy,drive=f0 $1 $2 $3 \
    -display curses

re:
	make clean
	make
