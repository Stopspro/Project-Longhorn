paint:     
kp:
	in al,60h
	test al,80h
	jz kp
     call clear_screen
     mov dl,0x0f
	 mov bx,msg10
	 call print_screen
	 mov di,1	 
     mov si,4
     call set_cursor
	 
    keypaint:
	 in al,60h
	 test al,80h
	 jNz keypaint
     
	 and al,7fh
	 
	 dec AL
 mov bx,tablepaint
	 xlat
	 cmp al,1
	 je exit_paint
	 cmp al,0
	 je keypaintup
	 cmp al,'1'
	 je paintblue
	 cmp al,'2'
	 je paintgreen
	 cmp al,'3'
	 je paintcyan
	 cmp al,'4'
	 je paintred
	 cmp al,'5'
	 je paintmagneta
	 cmp al,'6'
	 je paintbrown
	 cmp al,'7'
	 je paintyellow
	 cmp al,'8'
	 je paintwhite
	 cmp al,'9'
	 je eraser
	 cmp al,'a'
     je left
     cmp al,'d'
     je right	
     cmp al,'w'
     je up	
     cmp al,'s'
     je down		 
keypaintup:
	in al,60h
	test al,80h
	jz keypaintup
	jmp keypaint

eraser:
      mov dl,0x00
	  jmp keypaintup
paintwhite:
      mov dl,0x0f
	  jmp keypaintup
paintyellow:
      mov dl,0x0E
	  jmp keypaintup
paintbrown:
      mov dl,0x06
	  jmp keypaintup
paintmagneta:
      mov dl,0x05
	  jmp keypaintup
paintred:
      mov dl,0x04
	  jmp keypaintup
paintcyan:
      mov dl,0x03
	  jmp keypaintup	
paintgreen:
      mov dl,0x02
	  jmp keypaintup
paintblue:
      mov dl,0x01
      jmp keypaintup
	  
putchar_paint:
mov byte [gs:di],0x08
inc di
mov byte [gs:di],dl
inc di
ret 
