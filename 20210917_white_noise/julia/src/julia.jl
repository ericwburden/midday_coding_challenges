module julia
export encode, decode

char_morse_map = Dict{Char, String}(
    'A' => ".-",    'B' => "-...",  'C' => "-.-.",  'D' => "-..",
    'E' => ".",     'F' => "..-.",  'G' => "--.",   'H' => "....",
    'I' => "..",    'J' => ".---",  'K' => "-.-",   'L' => ".-..",
    'M' => "--",    'N' => "-.",    'O' => "---",   'P' => ".--.",
    'Q' => "--.-",  'R' => ".-.",   'S' => "...",   'T' => "-",
    'U' => "..-",   'V' => "...-",  'W' => ".--",   'X' => "-..-",
    'Y' => "-.--",  'Z' => "--..",  '0' => "-----", '1' => ".----",
    '2' => "..---", '3' => "...--", '4' => "....-", '5' => ".....",
    '6' => "-....", '7' => "--...", '8' => "---..", '9' => "----.",
    '#' => "-.----"
)


morse_char_map = Dict(value => key for (key, value) in char_morse_map)

dit_char = ' '
dah_char = '\t'
letter_sep = '\v'
word_sep = '\n'

function encodechar(c::Char)::String
    c == ' ' && return word_sep * letter_sep
    morsechars = collect(char_morse_map[c])
    out_string = map(x -> x == '.' ? dit_char : dah_char, morsechars) |> String
    out_string *= letter_sep
    return out_string
end

function decodemorse(m::AbstractString)::Char
  m == string(word_sep) && return ' '
  morsestring = map(x -> x == dit_char ? '.' : '-', collect(m)) |> String
  return morse_char_map[morsestring]
end

function encode(s::AbstractString)
    map(encodechar, collect(s)) |> join
end

function decode(s::AbstractString)
  whitespace_words = split(s, letter_sep, keepempty = false)
  map(decodemorse, whitespace_words) |> String
end

end # module
