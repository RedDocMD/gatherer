// This file tests lw and sw mainly
addi $t0, 0x10 // Mem addr
addi $t1, 14 // Data to store
sw $t1, 0($t0)
xor $t1, $t1 // Zero out $t1
addi $t1, 0x14
lw $t2, -4($t1) // This loads from $t0, effectively
