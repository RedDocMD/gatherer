main:
	xor $a0, $a0
	addi $a0, 8
	bl fibo
.L1:
	b .L1
fibo:
	xor $v0, $v0
	addi $v0, 1
	xor $t0, $t0
	addi $t0, 1
	comp $t1, $a0
	add $t0, $t1
	bltz $t0, .L8
	br $ra
.L8:
	push $fp
	push $s0
	addi $sp, -8
	mov $s0, $a0
	addi $a0, -1
	bl fibo
	mov $fp, $v0
	addi $s0, -2
	mov $a0, $s0
	bl fibo
	add $v0, $fp
	addi $sp, 8
	pop $s0
	pop $fp
	br $ra
