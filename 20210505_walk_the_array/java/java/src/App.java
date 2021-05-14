// Mid-Day Coding Challenge
// 
// You are given an array of non-negative integers. Let's say you start at the 
// beginning of the array and are trying to advance to the end. You can advance
// at most, the number of steps corresponding to the number that you're currently
// on. (For example, if the number at your current index is 3, you can advance 
// 1, 2, or 3 steps.) Determine whether you can get to the end of the array.
// 
// For example, given the array [1, 3, 1, 2, 0, 1], we can go from indices
// 0 -> 1 -> 3 -> 5, so return true.
// 
// Given the array [1, 2, 1, 0, 0], we can't reach the end, so return false.
// 
// Write a function that can take an array of arbitrary length and determine
// whether the array can be walked to the end as described.

import java.util.Arrays;
import static org.junit.jupiter.api.Assertions.assertEquals;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

public class App {

    static boolean canWalkArray(int[] array) {
        if (array.length == 1) return true;
        if (array[0] == 0) return false;
        for (int n = 1; n <= array[0]; n++) {
            int[] subArray = Arrays.copyOfRange(array, n, array.length);
            if (canWalkArray(subArray)) return true;
        }
        return false;
    }

    /**
     * Successful test case
     */
    @Test
    @DisplayName("[1, 3, 1, 2, 0, 1] can be walked.")
    public void testCaseOne() {
        int[] input = {1, 3, 1, 2, 0 , 1};
        boolean result = canWalkArray(input);
        assertEquals(true, result, "[1, 3, 1, 2, 0, 1] can be walked.");
    }

    /**
     * Failing test case
     */
    @Test
    @DisplayName("[1, 2, 1, 0, 0] cannot be walked.")
    public void testCaseTwo() {
        int[] input = {1, 2, 1, 0, 0};
        boolean result = canWalkArray(input);
        assertEquals(false, result, "[1, 2, 1, 0, 0] cannot be walked.");
    }
}

/*
    Compile with:
    javac -d out -cp lib/junit-platform-console-standalone-1.8.0-M1.jar src/App.java

    Run tests with:
    java -jar lib/junit-platform-console-standalone-1.8.0-M1.jar \
        --class-path out \
        --scan-class-path \
        --details tree \
        --include-classname '.*'
*/

