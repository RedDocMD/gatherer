// This code just keeps on adding
	addi $t0, 1000
Loop:
	compi $t1, 10
	add $t0, $t1
	b Loop
