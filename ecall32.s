    .text

	.section	.text.__ecall0,"ax",@progbits
	.globl	__ecall0
	.p2align	1
	.type	__ecall0,@function
__ecall0:
    addi a7, a0, 0
    ecall
