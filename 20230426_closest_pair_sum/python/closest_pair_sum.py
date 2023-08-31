import sys

class pairwise_diff_iter:
    def __init__(self, numbers: list[int], target: int) -> None:
        self.numbers = numbers
        self.target = target
        self.p1 = 0
        self.p2 = len(numbers) - 1
        self.diff = sys.maxsize

    def __iter__(self) -> 'pairwise_diff_iter':
        return self

    def __next__(self) -> tuple[int, int]:
        num1 = self.numbers[self.p1]
        num2 = self.numbers[self.p2]
        check = num1 + num2
        diff = abs(self.target - check)

        if self.p1 == self.p2 or diff > self.diff:
            raise StopIteration()
        elif check > self.target:
            self.p2 -= 1
        elif check < self.target:
            self.p1 += 1

        self.diff = diff
        return (num1, num2)


if __name__ == '__main__':
    numbers = [-2, 3, 5, 7, 10, 12]
    *_, last = pairwise_diff_iter(numbers, 6)
    print(last)



        
