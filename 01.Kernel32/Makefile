NASM32 = nasm
GCC32 = x86_64-linux-gnu-gcc -c -m32 -ffreestanding
LD32 = x86_64-linux-gnu-ld -melf_i386 -T ../elf_i386.x -nostdlib -e Main -Ttext 0x10200
OBJCOPY32 = x86_64-linux-gnu-objcopy -j .text -j .data -j .rodata -j .bss -S -O binary

LDFLAGS	=	-lrusty -L../Rust/target/i686-unknown-linux-gnu/release

OBJECTDIRECTORY = Temp
SOURCEDIRECTORY = Source

all:
	make RustyLibrary
	make Kernel32.bin

RustyLibrary:
	cd Rust && cargo build --release

$(OBJECTDIRECTORY)/EntryPoint.bin: $(SOURCEDIRECTORY)/EntryPoint.s
	mkdir -p $(OBJECTDIRECTORY)
	$(NASM32) -o $@ $<

dep:
	mkdir -p $(OBJECTDIRECTORY)
	make -C $(OBJECTDIRECTORY) -f ../Makefile InternalDependency

ExecuteInternalBuild: dep
	make -C $(OBJECTDIRECTORY) -f ../Makefile Kernel32.elf

$(OBJECTDIRECTORY)/Kernel32.elf.bin: ExecuteInternalBuild
	$(OBJCOPY32) $(OBJECTDIRECTORY)/Kernel32.elf $@

Kernel32.bin: $(OBJECTDIRECTORY)/EntryPoint.bin $(OBJECTDIRECTORY)/Kernel32.elf.bin
	cat $^ > $@

clean:
	rm -f *.bin
	rm -rf $(OBJECTDIRECTORY)

CENTRYPOINTOBJECTFILE = Main.o
CSOURCEFILES = $(wildcard ../$(SOURCEDIRECTORY)/*.c)
ASSEMBLYSOURCEFILES = $(wildcard ../$(SOURCEDIRECTORY)/*.asm)
COBJECTFILES = $(subst Main.o, , $(notdir $(patsubst %.c, %.o, $(CSOURCEFILES))))
ASSEMBLYOBJECTFILES = $(notdir $(patsubst %.asm, %.o, $(ASSEMBLYSOURCEFILES)))

%.o: ../$(SOURCEDIRECTORY)/%.c
	$(GCC32) -c $<

%.o: ../$(SOURCEDIRECTORY)/%.asm
	$(NASM32) -f elf32 -o $@ $<

InternalDependency:
	$(GCC32) -MM $(CSOURCEFILES) > Dependency.dep

Kernel32.elf: $(CENTRYPOINTOBJECTFILE) $(COBJECTFILES) $(ASSEMBLYOBJECTFILES)
	$(LD32) -o $@ $^ $(LDFLAGS)

ifeq (Dependency.dep, $(wildcard Dependency.dep))
include Dependency.dep
endif

.PHONY: clean all dep ExecuteInternalBuild InternelDependency
