import unittest
import time

def time_it(func):
    def st_func(*args, **keyArgs):
        t1 = time.time()
        r = func(*args, **keyArgs)
        t2 = time.time()
        print("Function=%s, Time=%0.8f" % (func.__name__, t2 - t1))
        return r
    return st_func

def pick_peak_linear(arr):
    """Solution in linear time"""
    for (idx, val) in enumerate(arr):
        prev = arr[idx - 1] if idx > 0 else float('-inf')
        next = arr[idx + 1] if idx < len(arr) - 1 else float('-inf')
        if prev < val > next:
            return val
    return None

def pick_peak_binary(arr):
    """Solution in log N time"""
    start, end = 0, len(arr) - 1
    while start < end:
        mid = start + (end - start) // 2

        if arr[mid-1] < arr[mid] > arr[mid+1]:
            return arr[mid]
        elif arr[mid] < arr[mid + 1]:
            start = mid + 1
        else:
            end = mid - 1

    return arr[start]
        

class TestPeakPicker(unittest.TestCase):
    def test_one(self):
        input = [1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1]
        assert pick_peak_linear(input) == 7
        assert pick_peak_binary(input) == 7

    def test_two(self):
        input = [7, 6, 5, 4, 3, 2, 1]
        assert pick_peak_linear(input) == 7
        assert pick_peak_binary(input) == 7

    def test_three(self):
        input = [1, 2, 3, 4, 5, 6, 7]
        assert pick_peak_linear(input) == 7
        assert pick_peak_binary(input) == 7

    def test_four(self):
        input = [7]
        assert pick_peak_linear(input) == 7
        assert pick_peak_binary(input) == 7


if __name__ == '__main__':
    big_list = range(1, 100000)
    time_it(pick_peak_linear)(big_list)
    time_it(pick_peak_binary)(big_list)

    # Run the unit tests
    unittest.main()
