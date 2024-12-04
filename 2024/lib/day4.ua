# $ MMMSXXMASM
# $ MSAMXMSMSA
# $ AMXSXMAAMM
# $ MSAMASMSMX
# $ XMASAMXAMM
# $ XXAMMXXAMA
# $ SMSMSASXSS
# $ SAXAMASAAA
# $ MAMMMXMMMM
# $ MXMXAXMASX
&fras "day4.txt"

⊜∘≠@\n.

# part 1
# diagonals of size 1, 2, .., side, side - 1, ..., 1
# D ← ⬚@O≡°□⊕□+-1⊞(-).⇡./↧△.
# combinations
# ⊂≡⇌.⊂⊂⊂∩D,,⇌⟜⍉

# /+≡(/+⌕"XMAS")


# part 2
# didn't know the existence of "stencil", otherwise should have used in part 1 :/
# create 3x3 windows, reshape into an n x 9 array
↯⊂⊃(/×↙2)(/×↘2)△.⧈∘ 3_3

# iterate each row, extract diagnals and their reverse
# check for MAS and then sum of the results
/+≡(×∩(+∩(/×="MAS")⟜⇌)⊃(⊏ [0 4 8])(⊏ [2 4 6]))
