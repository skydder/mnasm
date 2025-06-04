@nasm(BOOT_LOAD equ 0x7C00)
@nasm(ORG BOOT_LOAD)

<entry> {
    jmp(.ipl)
    <bpb> {
        @nasm(times 90 - ($ - $$) db 0x90)
    }
    <ipl> {
        cli()
        mov(ax, 0)
        @nasm(mov ds, ax)
        @nasm(mov es, ax)
        @nasm(mov ss, ax)
        @nasm(mov sp, BOOT_LOAD)
        @nasm(mov [BOOT.DRIVE], dl)
        
        mov(al, 65)
        mov(ah, 14)
        mov(bx, 0)
        int(16)
        
        @nasm(jmp $)
        @nasm(ALIGN 2, db 0)
        @nasm(BOOT:)
        @nasm(.DRIVE: dw 0)
    }
    <boot_flag> {
        @nasm(times 510 - ($ - $$) db 0x00)
        @nasm(db 0x55, 0xAA)
    }
}
