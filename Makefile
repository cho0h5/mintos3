all: Disk.img

BootLoader:
	make -C 00.BootLoader

Kernel32:
	make -C 01.Kernel32

Disk.img: BootLoader Kernel32
	cat 00.BootLoader/BootLoader.bin 01.Kernel32/Kernel32.bin > Disk.img

clean:
	make -C 00.BootLoader clean
	make -C 01.Kernel32 clean
	rm -f Disk.img

test: all
	qemu-system-x86_64 \
    -L . -m 64 -M pc \
    -blockdev driver=file,node-name=f0,filename=./Disk.img \
    -device floppy,drive=f0 $1 $2 $3 \
    -display curses
