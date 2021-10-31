import unittest

def div(a, b):
    if a == 0 or b == 0: return 0
    return int(round(a / b, 0))


def all_operations(a, b):
    return [a + b, a - b, a * b, div(a, b)]


def play_recursive(nums):
    if len(nums) == 1: return nums[0] == 24

    # For each pair of numbers in the input 
    for idx in range(len(nums) - 1):
        a, b = nums[idx:idx + 2]

        # For each possible result using that pair of numbers
        for result in all_operations(a, b):
            new_nums = nums[:idx] + [result] + nums[idx+2:]
            if play_recursive(new_nums): return True

    return False


def play(nums):
    stack = [nums]

    while stack:
        working_set = stack.pop()
        if len(working_set) == 1:
            if working_set[0] == 24: return True
            continue # Try the next item on the stack

        for idx in range(len(working_set) - 1):
            a, b = working_set[idx:idx + 2]

            for result in all_operations(a, b):
                next_set = working_set[:idx] + [result] + working_set[idx+2:]
                stack.append(next_set)

    return False


class TestPlay(unittest.TestCase):
    def setUp(self):
        self.prod_sub_prod = [5, 2, 7, 8]
        self.addition = [2, 4, 8, 10]
        self.subtraction = [27, 1, 1, 1]
        self.add_prod_add = [5, 0, 4, 4]
        self.div_roundup = [47, 2, 0, 0]
        self.div_rounddown = [1, 1, 73, 3]
        self.fail = [1, 5, 7, 19]

    def test_prod_sub_prod(self):
        self.assertTrue(play(self.prod_sub_prod), "(5 * 2 - 7) * 8 = 24 -> True")

    def test_addition(self):
        self.assertTrue(play(self.addition), "2 + 4 + 8 + 10 = 24 -> True")

    def test_subtraction(self):
        self.assertTrue(play(self.subtraction), "27 - 1 - 1 - 1 = 24 -> True")

    def test_add_prod_add(self):
        self.assertTrue(play(self.add_prod_add), "(5 + 0) * 4 + 4 = 24 -> True")

    def test_div_roundup(self):
        self.assertTrue(play(self.div_roundup), "47 / 2 + 0 + 0 = 23.5 -> 24 -> True")

    def test_div_rounddown(self):
        self.assertTrue(play(self.div_rounddown), "1 - 1 + (73 / 3) = 24.33 -> 24 -> True")

    def test_fail(self):
        self.assertFalse(play(self.fail), "1 ? 5 ? 7 ? 19 != 24 -> False")

    def tearDown(self):
        pass

if __name__ == "__main__":
    unittest.main()

    
