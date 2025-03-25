	extern printf, assert_
section .data
global GL_L_L_3
GL_L_L_3:
	db 110, 117, 109, 10, 0

section .data
global GL_L_L_2
GL_L_L_2:
	db 102, 105, 122, 122, 10, 0

section .data
global GL_L_L_1
GL_L_L_1:
	db 98, 117, 122, 122, 10, 0

section .data
global GL_L_L_0
GL_L_L_0:
	db 102, 105, 122, 122, 98, 117, 122, 122, 10, 0

section .text
global main
main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	lea rax, [rbp - 16]
	push rax
	mov rax, 1
	pop rdi
	mov dword [rdi], eax
main__N_L_L_1__L_begin_1:

	mov rax, 40
	push rax
	lea rax, [rbp - 16]
	movsx rax, dword [rax]
	pop rdi
	cmp eax, edi
	setl al
	movsx rax, al
	cmp rax, 0
	je main__N_L_L_1__L_end_1
	lea rax, [rbp - 12]
	push rax
	mov rax, 0
	push rax
	mov rax, 3
	push rax
	lea rax, [rbp - 16]
	movsx rax, dword [rax]
	pop rdi
	cdq
	idiv edi
	mov eax, edx
	pop rdi
	cmp eax, edi
	sete al
	movsx rax, al
	pop rdi
	mov dword [rdi], eax
	lea rax, [rbp - 8]
	push rax
	mov rax, 0
	push rax
	mov rax, 5
	push rax
	lea rax, [rbp - 16]
	movsx rax, dword [rax]
	pop rdi
	cdq
	idiv edi
	mov eax, edx
	pop rdi
	cmp eax, edi
	sete al
	movsx rax, al
	pop rdi
	mov dword [rdi], eax
	lea rax, [rbp - 4]
	push rax
	lea rax, [rbp - 8]
	movsx rax, dword [rax]
	push rax
	lea rax, [rbp - 12]
	movsx rax, dword [rax]
	pop rdi
	and eax, edi
	pop rdi
	mov dword [rdi], eax
	lea rax, [rbp - 4]
	movsx rax, dword [rax]
	cmp rax, 0
	je main__N_L_L_1__N_L_L_23__L_else_2
	mov rax, GL_L_L_0
	push rax
	pop rdi
	mov rax, 0
	call printf
	jmp main__N_L_L_1__N_L_L_23__L_end_2
main__N_L_L_1__N_L_L_23__L_else_2:
	lea rax, [rbp - 8]
	movsx rax, dword [rax]
	cmp rax, 0
	je main__N_L_L_1__N_L_L_23__L_else_2__N_L_L_27__L_else_3
	mov rax, GL_L_L_1
	push rax
	pop rdi
	mov rax, 0
	call printf
	jmp main__N_L_L_1__N_L_L_23__L_else_2__N_L_L_27__L_end_3
main__N_L_L_1__N_L_L_23__L_else_2__N_L_L_27__L_else_3:
	lea rax, [rbp - 12]
	movsx rax, dword [rax]
	cmp rax, 0
	je main__N_L_L_1__N_L_L_23__L_else_2__N_L_L_27__L_else_3__N_L_L_31__L_else_4
	mov rax, GL_L_L_2
	push rax
	pop rdi
	mov rax, 0
	call printf
	jmp main__N_L_L_1__N_L_L_23__L_else_2__N_L_L_27__L_else_3__N_L_L_31__L_end_4
main__N_L_L_1__N_L_L_23__L_else_2__N_L_L_27__L_else_3__N_L_L_31__L_else_4:
	mov rax, GL_L_L_3
	push rax
	pop rdi
	mov rax, 0
	call printf

main__N_L_L_1__N_L_L_23__L_else_2__N_L_L_27__L_else_3__N_L_L_31__L_end_4:


main__N_L_L_1__N_L_L_23__L_else_2__N_L_L_27__L_end_3:


main__N_L_L_1__N_L_L_23__L_end_2:

	lea rax, [rbp - 16]
	push rax
	mov rax, 1
	push rax
	lea rax, [rbp - 16]
	movsx rax, dword [rax]
	pop rdi
	add eax, edi
	pop rdi
	mov dword [rdi], eax
	jmp main__N_L_L_1__L_begin_1
main__N_L_L_1__L_end_1:

main__L_return_main:
	mov rsp, rbp
	pop rbp
	ret


extern printf