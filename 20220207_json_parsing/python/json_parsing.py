import json

def read_input():
    with open("input.txt", "r") as f:
        input = json.load(f)
    return input


def parse_numbers(input, numbers=[]):
    if isinstance(input, dict):
        for value in input.values():
            parse_numbers(value, numbers)

    if isinstance(input, list):
        for value in input:
            parse_numbers(value, numbers)

    if isinstance(input, int):
        numbers.append(input)
    
    return numbers


def sum_numbers(input):
    if isinstance(input, int):  return input
    if isinstance(input, dict): return sum(sum_numbers(v) for v in input.values())
    if isinstance(input, list): return sum(sum_numbers(v) for v in input)
    return 0


def parse_numbers2(input, numbers=[]):
    if isinstance(input, dict):
        if "red" in input.values(): return numbers
        for value in input.values():
            parse_numbers2(value, numbers)

    if isinstance(input, list):
        for value in input:
            parse_numbers2(value, numbers)

    if isinstance(input, int):
        numbers.append(input)
    
    return numbers


def sum_numbers2(input):
    if isinstance(input, int):  return input
    if isinstance(input, list): return sum(sum_numbers2(v) for v in input)
    if isinstance(input, dict):
        if "red" in input.values(): return 0
        return sum(sum_numbers2(v) for v in input.values())
    return 0


if __name__ == "__main__":
    input = read_input()
    answer1 = sum(parse_numbers(input))
    answer2 = sum(parse_numbers2(input))

    print("Results:")
    print(f"  Part 1: {answer1}")
    print(f"  Part 2: {answer2}")
