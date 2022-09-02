# Implementation --------------------------------------------------------------

transpose_builtin <- \(x) t(x)

transpose_byhand <- function(mx) {
    is.matrix(mx) |> stopifnot() # Type checking

    mx_dim <- dim(mx)
    out    <- matrix(NA, mx_dim[2], mx_dim[1])

    for (row in seq_len(mx_dim[1])) {
        for (col in seq_len(mx_dim[2])) {
            out[col, row] <- mx[row, col]
        }
    }

    out
}


# Tests -----------------------------------------------------------------------

require(testthat)

test_that("Empty matrix is unchanged", {
    empty_matrix <- matrix(NA, 0, 0)
    dim(empty_matrix) |> expect_equal(c(0, 0))
    transpose_builtin(empty_matrix) |> expect_identical(empty_matrix)
    transpose_byhand(empty_matrix)  |> expect_identical(empty_matrix)
})

test_that("Single-item matrix is unchanged", {
    single_item <- matrix(1, 1, 1)
    is.matrix(single_item) |> expect_true()
    dim(single_item) |> expect_equal(c(1, 1))
    transpose_builtin(single_item) |> expect_identical(single_item)
    transpose_byhand(single_item)  |> expect_identical(single_item)
})

test_that("3x1 example is correctly transposed to 1x3", {
    input  <- matrix(1:3, 1, 3, byrow = TRUE)
    expect <- matrix(1:3, 3, 1)
    transpose_builtin(input) |> expect_identical(expect)
    transpose_byhand(input)  |> expect_identical(expect)

})

test_that("3x2 example is correctly transposed to 2x3", {
    input  <- matrix(1:6, 2, 3, byrow = TRUE)
    expect <- matrix(1:6, 3, 2)
    transpose_builtin(input) |> expect_identical(expect)
    transpose_byhand(input)  |> expect_identical(expect)
})

test_that("2x3 example is correctly transposed to 3x2", {
    input  <- matrix(1:6, 3, 2, byrow = TRUE)
    expect <- matrix(1:6, 2, 3)
    transpose_builtin(input) |> expect_identical(expect)
    transpose_byhand(input)  |> expect_identical(expect)
})

test_that("3x4 example is correctly transposed to 4x3", {
    input  <- matrix(1:12, 3, 4, byrow = TRUE)
    expect <- matrix(1:12, 4, 3)
    transpose_builtin(input) |> expect_identical(expect)
    transpose_byhand(input)  |> expect_identical(expect)
})
