c:\
c:\python27\python.exe c:\Longhorn\harold.py %*
cd c:\Longhorn\boot\x86-64
nasm -f elf64 multiboot_header.asm
nasm -f elf64 boot.asm
ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o
cd c:\
exit /b
