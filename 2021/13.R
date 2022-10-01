library(dplyr)
library(purrr)

# Utils -------------------------------------------------------------------

fold_matrix <- function(m, axis = c("x", "y"), where) {
  axis <- match.arg(axis)
  if (axis == "y") m <- t(m)

  side1 <- m[, 1:(where - 1), drop = FALSE]
  side2 <- m[, ncol(m):(where + 1), drop = FALSE]

  if (ncol(side1) != ncol(side2)) {
    filler <- matrix(0L, nrow = nrow(m), ncol = abs(ncol(side1) - ncol(side2)))

    if (ncol(side1) < ncol(side2)) side1 <- cbind(filler, side1)
    else side2 <- cbind(filler, side2)
  }

  m <- side1 + side2
  m[m > 1L] <- 1L

  if (axis == "y") t(m) else m
}

# Data --------------------------------------------------------------------

input <- readLines(here::here("2021/13-input"))

dots <- tibble(coord = input[1:(which(input == "") - 1)]) |>
  tidyr::separate(coord, into = c("x", "y"), sep = ",", convert = TRUE) |>
  # To 1-based
  mutate(across(c(x, y), ~ .x + 1L))

folds <- input[(which(input == "") + 1):length(input)] |>
  sub(pattern = "fold along ", replacement = "") |>
  as_tibble() |>
  tidyr::separate(value, into = c("axis", "value"), convert = TRUE) |>
  # To 1-based
  mutate(value = value + 1L)

sheet <- matrix(0L, nrow = max(dots$y), ncol = max(dots$x))

pwalk(dots, \(x, y) {
  sheet[y, x] <<- 1L
})

# Part 1 ------------------------------------------------------------------

folds |>
  slice(1) |>
  pmap(\(...) fold_matrix(sheet, ..1, ..2)) |>
  pluck(1) |>
  sum()

# Part 2 ------------------------------------------------------------------

pwalk(folds, \(...) {
  sheet <<- fold_matrix(sheet, ..1, ..2)
})

ragg::agg_png(here::here("2021/13-answer.png"), background = "black")
sheet |> as.raster() |> plot()
dev.off()
