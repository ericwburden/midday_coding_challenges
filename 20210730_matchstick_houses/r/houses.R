make_houses <- function(n) {
  if (n == 0) 0 else (n * 5) + 1
}

#' Draw a matchstick house to current graphics device
#'
#' @param offset x/y offset, starting with bottom left = 0
#' @param scale scale adjustment of the house, house size decreases as scale increases
draw_house <- function(offset = 0, scale = 1) {
  xoffset = (offset %% (10 * scale))/10
  yoffset = (offset %/% (10 * scale))/ 2

  # Coordinates for the box
  xcoords = (c(0.0, 0.0, 0.1, 0.1) + xoffset)/scale
  ycoords = (c(0.0, 0.2, 0.2, 0.0) + yoffset)/scale
  boxcoords = matrix(c(xcoords, ycoords), ncol = 2)

  # Draw the box matchsticks
  polygon(boxcoords, lwd=4, border = "#e4d5bc")

  # Draw the triangle
  xcoords = (c(0.0, .05, 0.1) + xoffset)/scale
  ycoords = (c(0.2, .35, 0.2) + yoffset)/scale
  tricoords = matrix(c(xcoords, ycoords), ncol = 2)
  polygon(tricoords, lwd=4, border = "#e4d5bc")

  # Draw all the match heads
  polygon(boxcoords[c(1, 1), ], lwd=8, border = "#FF0000")
  polygon(boxcoords[c(2, 2), ], lwd=8, border = "#FF0000")
  polygon(boxcoords[c(3, 3), ], lwd=8, border = "#FF0000")
  polygon(boxcoords[c(4, 4), ], lwd=8, border = "#FF0000")
  polygon(tricoords[c(2, 2), ], lwd=8, border = "#FF0000")
}

#' Draw a number of matchstick houses
#'
#' @param n number of houses to draw
#' @param save_as optional, filename to save a PNG image to
draw_houses <- function(n, save_as = NULL) {
  stopifnot(n <= 320)
  scale = if (n <= 20) 1 else if (n <= 80) 2 else if (n <= 180) 3 else 4
  par(bg = "black")
  if (is.null(save_as)) plot.new() else png(file = save_as, bg = "black")
  plot.new()
  lapply(0:(n-1), draw_house, scale = scale)
  if (!is.null(save_as)) dev.off()
}
