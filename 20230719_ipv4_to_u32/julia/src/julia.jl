module julia

function int32toip(n::UInt32)::String
  # Get the leftmost 8 bits and shift them right by 24 bits
  part1 = (n & 0b11111111000000000000000000000000) >> 24

  # Get the next 8 bits and shift them right by 16 bits
  part2 = (n & 0b00000000111111110000000000000000) >> 16

  # Get the next 8 bits and shift them right by 8 bits
  part3 = (n & 0b00000000000000001111111100000000) >> 8

  # Get the rightmost 8 bits
  part4 = (n & 0b00000000000000000000000011111111)

  # Return a formatted string
  "$part1.$part2.$part3.$part4"
end

end # module julia
