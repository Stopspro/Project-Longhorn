c:\
cd c:\Longhorn
del "README.md"
del "longhorn.jpg"
del "LICENSE"
del "CODE_OF_CONDUCT.md"
del "DEVELOPING.md"
cd c:\Longhorn\boot
del "shellcommands.txt"
cd c:\Longhorn\packages\paint
del "paint.asm"
cd c:\Longhorn\gui
del "tempfile"
cd c:\Longhorn\BMFS
git clone https://github.com/ReturnInfinity/BMFS
cd c:\Longhorn\boot\x86-64
nasm -f elf64 multiboot_header.asm
nasm -f elf64 boot.asm
ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o
cd c:\Longhorn
RUST_TARGET_PATH=$(pwd) xargo build --target=x86_64-longhorn_os
start /d "c:\Longhorn" start-shell.exe
exit /b
