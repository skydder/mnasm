global prep
prep:
	extern printfassert_
section .data
global L_L_26
L_L_26:
	db 79797979
section .data
global L_L_25
L_L_25:
	db 4949494949
section .data
global L_L_24
L_L_24:
	db 4949494949
section .data
global L_L_23
L_L_23:
	db 4949494949
section .data
global L_L_22
L_L_22:
	db 49494949
section .data
global L_L_21
L_L_21:
	db 49494949
section .data
global L_L_20
L_L_20:
	db 49494949
section .data
global L_L_19
L_L_19:
	db 5050505050
section .data
global L_L_18
L_L_18:
	db 4949494949
section .data
global L_L_17
L_L_17:
	db 4848484848
section .data
global L_L_16
L_L_16:
	db 50505050
section .data
global L_L_15
L_L_15:
	db 49494949
section .data
global L_L_14
L_L_14:
	db 48484848
section .data
global L_L_13
L_L_13:
	db 52525252525252
section .data
global L_L_12
L_L_12:
	db 4848484848
section .data
global L_L_11
L_L_11:
	db 52525252525252
section .data
global L_L_10
L_L_10:
	db 4848484848
section .data
global L_L_9
L_L_9:
	db 4545454545454545
section .data
global L_L_8
L_L_8:
	db 454545454545
section .data
global L_L_7
L_L_7:
	db 45454545454545
section .data
global L_L_6
L_L_6:
	db 4040404040404040
section .data
global L_L_5
L_L_5:
	db 5353535353535353
section .data
global L_L_4
L_L_4:
	db 535353535353
section .data
global L_L_3
L_L_3:
	db 494949494949494949494949
section .data
global L_L_2
L_L_2:
	db 53535353535353
section .data
global L_L_1
L_L_1:
	db 525252
section .data
global L_L_0
L_L_0:
	db 4848
section .text
global main
main:
	push rbp
	mov rbp, rsp
	sub rsp, 0
	mov rax, 0
	push rax
	mov rax, 0
	push rax
	mov rax, L_L_0
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 42
	push rax
	mov rax, 42
	push rax
	mov rax, L_L_1
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 21
	push rax
	mov rax, 4
	push rax
	mov rax, 20
	push rax
	mov rax, 5
	pop rdi
	add rax, rdi
	pop rdi
	sub rax, rdi
	push rax
	mov rax, L_L_2
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 41
	push rax
	mov rax, 5
	push rax
	mov rax, 34
	push rax
	mov rax, 12
	pop rdi
	add rax, rdi
	pop rdi
	sub rax, rdi
	push rax
	mov rax, L_L_3
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 47
	push rax
	mov rax, 7
	push rax
	mov rax, 6
	pop rdi
	imul rax, rdi
	push rax
	mov rax, 5
	pop rdi
	add rax, rdi
	push rax
	mov rax, L_L_4
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 15
	push rax
	mov rax, 6
	push rax
	mov rax, 9
	pop rdi
	sub rax, rdi
	push rax
	mov rax, 5
	pop rdi
	imul rax, rdi
	push rax
	mov rax, L_L_5
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 4
	push rax
	mov rax, 2
	push rax
	mov rax, 5
	push rax
	mov rax, 3
	pop rdi
	add rax, rdi
	pop rdi
	cqo
	idiv rdi
	push rax
	mov rax, L_L_6
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 10
	push rax
	mov rax, 20
	push rax
	mov rax, 10
	neg rax
	pop rdi
	add rax, rdi
	push rax
	mov rax, L_L_7
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 10
	push rax
	mov rax, 10
	neg rax
	neg rax
	push rax
	mov rax, L_L_8
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 10
	push rax
	mov rax, 10
	neg rax
	neg rax
	push rax
	mov rax, L_L_9
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 0
	push rax
	mov rax, 1
	push rax
	mov rax, 0
	pop rdi
	cmp rax, rdi
	sete al
	movzb rax, al
	push rax
	mov rax, L_L_10
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 1
	push rax
	mov rax, 42
	push rax
	mov rax, 42
	pop rdi
	cmp rax, rdi
	sete al
	movzb rax, al
	push rax
	mov rax, L_L_11
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 1
	push rax
	mov rax, 1
	push rax
	mov rax, 0
	pop rdi
	cmp rax, rdi
	setne al
	movzb rax, al
	push rax
	mov rax, L_L_12
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 0
	push rax
	mov rax, 42
	push rax
	mov rax, 42
	pop rdi
	cmp rax, rdi
	setne al
	movzb rax, al
	push rax
	mov rax, L_L_13
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 1
	push rax
	mov rax, 1
	push rax
	mov rax, 0
	pop rdi
	cmp rax, rdi
	setl al
	movzb rax, al
	push rax
	mov rax, L_L_14
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 0
	push rax
	mov rax, 1
	push rax
	mov rax, 1
	pop rdi
	cmp rax, rdi
	setl al
	movzb rax, al
	push rax
	mov rax, L_L_15
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 0
	push rax
	mov rax, 1
	push rax
	mov rax, 2
	pop rdi
	cmp rax, rdi
	setl al
	movzb rax, al
	push rax
	mov rax, L_L_16
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 1
	push rax
	mov rax, 1
	push rax
	mov rax, 0
	pop rdi
	cmp rax, rdi
	setle al
	movzb rax, al
	push rax
	mov rax, L_L_17
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 1
	push rax
	mov rax, 1
	push rax
	mov rax, 1
	pop rdi
	cmp rax, rdi
	setle al
	movzb rax, al
	push rax
	mov rax, L_L_18
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 0
	push rax
	mov rax, 1
	push rax
	mov rax, 2
	pop rdi
	cmp rax, rdi
	setle al
	movzb rax, al
	push rax
	mov rax, L_L_19
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 1
	push rax
	mov rax, 1
	push rax
	mov rax, 0
	pop rdi
	cmp rax, rdi
	setl al
	movzb rax, al
	push rax
	mov rax, L_L_20
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 0
	push rax
	mov rax, 1
	push rax
	mov rax, 1
	pop rdi
	cmp rax, rdi
	setl al
	movzb rax, al
	push rax
	mov rax, L_L_21
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 0
	push rax
	mov rax, 1
	push rax
	mov rax, 2
	pop rdi
	cmp rax, rdi
	setl al
	movzb rax, al
	push rax
	mov rax, L_L_22
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 1
	push rax
	mov rax, 1
	push rax
	mov rax, 0
	pop rdi
	cmp rax, rdi
	setle al
	movzb rax, al
	push rax
	mov rax, L_L_23
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 1
	push rax
	mov rax, 1
	push rax
	mov rax, 1
	pop rdi
	cmp rax, rdi
	setle al
	movzb rax, al
	push rax
	mov rax, L_L_24
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, 0
	push rax
	mov rax, 1
	push rax
	mov rax, 2
	pop rdi
	cmp rax, rdi
	setle al
	movzb rax, al
	push rax
	mov rax, L_L_25
	push rax
	pop rdx
	pop rsi
	pop rdi
	mov rax, 0
	call assert_
	mov rax, L_L_26
	push rax
	pop rdi
	mov rax, 0
	call printf
	mov rax, 0
	jmp L_return_main__main__
L_return_main__main__:
	mov rsp, rbp
	pop rbp
	ret


