main:
	addi $a0, 21
	bl quadruple
	xor $a0, $a0
	add $a0, $v0
quadruple:
	xor $t0, $t0
	add $t0, $a0
	sll $t0, 2
	xor $v0, $v0
	add $v0, $t0
	br $ra
