CHAR_MORSE="A:.- B:-... C:-.-. D:-.. E:. F:..-. G:--. H:.... I:.. J:.--- K:-.-
            L:.-.. M:-- N:-. O:--- P:.--. Q:--.- R:.-. S:... T:- U:..- V:...-
            W:.-- X:-..- Y:-.-- Z:--.. 0:----- 1:.---- 2:..--- 3:...-- 4:....-
            5:..... 6:-.... 7:--... 8:---.. 9:----." 
CHAR_MORSE_MAP=$(echo $CHAR_MORSE | tr ' ' '\n')
INPUT=$(echo $1 | tr ' ' '\n' | sed 's/\([[:alnum:]]\)/\1 /g')
MORSE_OUTPUT=""

IFS=$'\n'
for word in $INPUT; do
    IFS=' '
    for char in $word; do
        MORSE=$(echo $CHAR_MORSE_MAP | grep $char | cut -c 3-)
        MORSE_OUTPUT+="$MORSE|"
    done
    MORSE_OUTPUT+="\n"
done
unset IFS
# echo -e $MORSE_OUTPUT

echo -e $MORSE_OUTPUT | tr '|.-' '\v \t' 
