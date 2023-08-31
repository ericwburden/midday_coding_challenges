adjacent('1', X):- X = '1'; X = '2'; X = '4'.
adjacent('2', X):- X = '2'; X = '1'; X = '3'; X = '5'.
adjacent('3', X):- X = '3'; X = '2'; X = '6'.
adjacent('4', X):- X = '4'; X = '1'; X = '5'; X = '7'.
adjacent('5', X):- X = '5'; X = '2'; X = '4'; X = '6'; X = '8'.
adjacent('6', X):- X = '6'; X = '3'; X = '5'; X = '9'.
adjacent('7', X):- X = '7'; X = '4'; X = '8'.
adjacent('8', X):- X = '8'; X = '5'; X = '7'; X = '9'; X = '0'.
adjacent('9', X):- X = '9'; X = '6'; X = '8'.
adjacent('0', X):- X = '0'; X = '8'.

alternate_keys([], []).
alternate_keys([H | R], AlternateKeys):- 
  alternate_keys(R, PartialKeys),
  adjacent(H, KeyOption),
  AlternateKeys = [KeyOption | PartialKeys].

alternate_pin(ObservedPin, PossiblePin):-
  string_chars(ObservedPin, ObservedKeys),
  alternate_keys(ObservedKeys, PossiblePinKeys),
  string_chars(PossiblePin, PossiblePinKeys).

get_pins(ObservedPin, PossiblePins):-
  findall(A, alternate_pin(ObservedPin, A), PossiblePins).
