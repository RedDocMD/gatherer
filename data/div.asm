// Perform division by repeated subtraction
	addi $t0, 45 // Dividend
	addi $t1, 3 // Divisor
	addi $t2, 0 // Quotient
// if t0 < t1, then exit
Loop:
	comp $t3, $t1
	add $t3, $t0
	bltz $t3, Exit
// Loop body
	addi $t2, 1 // Increment quotient
	comp $t3, $t1
	add $t0, $t3
	b Loop
Exit:
