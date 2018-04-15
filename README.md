# Project Longhorn
![alt text](https://raw.githubusercontent.com/barely-functioning/Project-Longhorn/master/longhorn.jpg)

Project Longhorn is nothing more than a project. The goal of said project is to make the world's smallest OS. Our goal is to be able to boot from a 1.44 MB floppy drive, and to be as fast as possible (while using very little RAM).

Project Longhorn is written in Rust, C, and Assembly (but mainly C). We chose Rust as the language of choice because not only are there very few OS's written in Rust, it is also pretty easy to learn, and as a result, it is very easy to customize the OS. Although, we wanted a simple filesystem, that way anyone interested in modding the OS could easily understand the code. So, we chose the FatFS filesystem, which turned our project from 82% Rust to 98% C. 

