c:\
c:\python27\python.exe c:\atomOS\removedocs.py %*
cd c:\atomOS\boot\x86-64
nasm -f elf64 multiboot_header.asm
nasm -f elf64 boot.asm
ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o
pause
