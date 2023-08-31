from collections import deque


class PinCracker:
    pin_mapping = {
        "1": ["1", "2", "4"],
        "2": ["2", "1", "3", "5"],
        "3": ["3", "2", "6"],
        "4": ["4", "1", "5", "7"],
        "5": ["5", "2", "4", "6", "8"],
        "6": ["6", "3", "5", "9"],
        "7": ["7", "4", "8"],
        "8": ["8", "5", "7", "9", "0"],
        "9": ["9", "6", "8"],
        "0": ["0", "8"],
    }

    def __init__(self, observed_pin: str):
        if not observed_pin.isdigit():
            raise ValueError("input must be string of digits")
        if not 1 <= len(observed_pin) <= 8:
            raise ValueError("input must be between 1-8 digits")
        self.observed_pin = observed_pin

    def get_pins(self):
        # Our queue for a breadth-first search. Each item contained is a
        # tuple of a partial pin and the index of the next key to check for
        # options. We start with a partial pin with no keys and the index
        # of the first set of options.
        partial_pins = deque[tuple[str, int]]([("", 0)])

        # List of completed pins
        complete_pins = []

        # List of lists, all options for each pin key in order
        key_options = [self.pin_mapping[c] for c in self.observed_pin]

        # Typical breadth-first search, keep searching while the queue
        # still has items to look through
        while partial_pins:
            # Pop off the last item in the queue
            template, key_idx = partial_pins.pop()

            # For each option at the appropriate index, create a new pin
            # (maybe partial, maybe complete) by appending that option to
            # the previous partial pin. If the pin is complete (has the
            # same length as the input pin), then we add it to the list of
            # complete pins.
            for option in key_options[key_idx]:
                next_pin = template + option
                if len(self.observed_pin) == len(next_pin):
                    complete_pins.append(next_pin)
                else:
                    partial_pins.append((next_pin, key_idx + 1))

        return complete_pins


if __name__ == "__main__":
    # fmt: off
    test_case_one   = (  "8", ["5", "7", "8", "9", "0"])
    test_case_two   = ( "11", ["11", "22", "44", "12", "21", "14", "41", "24", "42"])
    test_case_three = ("369", ["339", "366", "399", "658", "636", "258", "268", "669",
                               "668", "266", "369", "398", "256", "296", "259", "368",
                               "638", "396", "238", "356", "659", "639", "666", "359",
                               "336", "299", "338", "696", "269", "358", "656", "698",
                               "699", "298", "236", "239"])
    test_cases = [test_case_one, test_case_two, test_case_three]

    for input, output in test_cases:
        result = PinCracker(input).get_pins()
        assert set(result) == set(output)

    print("All tests passed!")
