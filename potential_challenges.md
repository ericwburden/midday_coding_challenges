# Stepping Down

Given a positive integer N, find the smallest number of steps it will take to reach 1.

There are two kinds of permitted steps:

- You may decrement N to N - 1.
- If a * b = N, you may decrement N to the larger of a and b.

For example, given 100, you can reach 1 in five steps with the following route: 
`100 -> 10 -> 9 -> 3 -> 2 -> 1`.


# Trouble with Toeplitz

In linear algebra, a Toeplitz matrix is one in which the elements on any given diagonal
from top left to bottom right are identical.

Here is an example:

```
1 2 3 4 8
5 1 2 3 4
4 5 1 2 3
7 4 5 1 2

```

Write a program to determine whether a given input is a Toeplitz matrix.


# Shape Shifter

You are given a 2 x N board, and instructed to completely cover the board with the
following shapes:

- Dominoes, or 2 x 1 rectangles.
- Trominoes, or L-shapes.

For example, if N = 4, here is one possible configuration, where A is a domino, and B
and C are trominoes.

```
A B B C
A B C C

```

Given an integer N, determine in how many ways this task is possible.


# Voter Verification

On election day, a voting machine writes data in the form (voter_id, candidate_id) to
a text file. Write a program that reads this file as a stream and returns the top 3
candidates at any given time. If you find a voter voting more than once, report this
as fraud.
