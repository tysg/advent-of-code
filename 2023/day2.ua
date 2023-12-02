&fras "day2.txt"

Num ← ⊢⇌\(+×10)▽↧≥0,<10.-@0

Color ← /↥⍉≡(⊐⌕){"red" "green" "blue"}
SetOfCube ← ×⊃Color Num
ParseLine ← ∵(/+∵SetOfCube⊜□≠,@,⊔)⊜□≠,@;⊔⊢↘1⊜□≠,@:

PartOne ← /+×+1⇡⧻.∵(/↧/↧ ≡(≤0 -[12 13 14])ParseLine ⊔) ⊜□≠,@\n
PartTwo ← /+∵(/×/↥ ParseLine ⊔) ⊜□≠,@\n
