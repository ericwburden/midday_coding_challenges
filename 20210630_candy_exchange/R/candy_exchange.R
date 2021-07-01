# You are given a certain number of candies and an exchange rate. For every exchange
# number of candy wrappers that you trade in, you receive an additional candy. Return
# the maximum number of candies that you can eat.  Note: Each candy is wrapped in a
# candy wrapper.
#
# Ex: Given the following candies and exchange…
#   candies = 3, exchange = 3, return 4 (each your three candies, exchange 3 wrappers,
#   each additional candy).
#
# Ex: Given the following candies and exchange…
#   candies = 3, exchange = 4, return 3.

candy_exchange <- function(candies, exchange_rate) {
    stopifnot(is.numeric(candies) & length(candies) == 1)
    stopifnot(is.numeric(exchange_rate) & length(exchange_rate) == 1)
    if (exchange_rate <= 1) return(Inf) # Infinite candy!

    wrappers <- candies
    while (wrappers > 0) {
        exchanged_candies <- wrappers %/% exchange_rate
        candies <- candies + exchanged_candies
        wrappers <- exchanged_candies
    }   
    candies
}

require(testthat)
test_that("Can correctly calculate candies", {
    expect_equal(candy_exchange(3, 3), 4)
    expect_equal(candy_exchange(3, 4), 3)
    expect_equal(candy_exchange(10, 2), 18)
    expect_equal(candy_exchange(1, 1), Inf)
})

test_that("Errors are thrown", {
    expect_error(candy_exchange("3", 5))
    expect_error(candy_exchange(1:10, 5))
    expect_error(candy_exchange(10, "5"))
    expect_error(candy_exchange(10, 1:5))
})
