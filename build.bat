c:\
c:\python27\python.exe c:\atomOS\removedocs.py %*
cd c:\atomOS\boot\x86-64
nasm -f elf64 multiboot_header.asm
nasm -f elf64 boot.asm
ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o
VBoxManage startvm "Ubuntu" --type headless
sleep 300
VBoxManage controlvm "Ubuntu" poweroff --type headless
VBoxManage startvm "Atom" --type headless
sleep 300
VBoxManage controlvm "Ubuntu" poweroff --type headless
exit /b
