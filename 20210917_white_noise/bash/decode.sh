CHAR_MORSE="A:.- B:-... C:-.-. D:-.. E:. F:..-. G:--. H:.... I:.. J:.--- K:-.-
            L:.-.. M:-- N:-. O:--- P:.--. Q:--.- R:.-. S:... T:- U:..- V:...-
            W:.-- X:-..- Y:-.-- Z:--.. 0:----- 1:.---- 2:..--- 3:...-- 4:....-
            5:..... 6:-.... 7:--... 8:---.. 9:----." 
CHAR_MORSE_MAP=$(echo $CHAR_MORSE | tr ' ' '\n')
INPUT=$(echo "$1" | tr '\v \t' ' .-')
ALPHANUMERIC_OUTPUT=""

IFS=$'\n'
for word in $INPUT; do
    IFS=' '
    for morse in $word; do
        ESCAPED_DOTS=$(echo $morse | sed 's/\./\\./g')
        SEARCH_STRING=":$ESCAPED_DOTS$"
        ALPHANUM=$(echo $CHAR_MORSE_MAP | grep $SEARCH_STRING | cut -c -1)
        ALPHANUMERIC_OUTPUT+="$ALPHANUM "
    done
    ALPHANUMERIC_OUTPUT+="\n"
done
unset IFS

echo -e $ALPHANUMERIC_OUTPUT | tr -d ' ' | tr '\n' ' ' | xargs
