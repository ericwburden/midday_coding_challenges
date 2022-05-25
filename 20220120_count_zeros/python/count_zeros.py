from typing import Iterator


def digits(number:int) -> Iterator[int]: 
    while number > 0:
       number, digit = divmod(number, 10) 
       yield digit


def count_zeros_in_number(number:int) -> int:
    return list(digits(number)).count(0)


def count_zeros_in_range(start:int, stop:int, step:int=1) -> int:
    return sum(count_zeros_in_number(n) for n in range(start, stop, step))
