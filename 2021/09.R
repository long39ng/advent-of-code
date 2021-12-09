library(terra)
library(purrr)
library(dplyr)

# Utils -------------------------------------------------------------------

is_lowpoint <- function(m, row, col) {
  adjacents <- c(
    tryCatch(m[row - 1, col], error = \(e) integer()),
    tryCatch(m[row + 1, col], error = \(e) integer()),
    tryCatch(m[row, col - 1], error = \(e) integer()),
    tryCatch(m[row, col + 1], error = \(e) integer())
  )
  sum(m[row, col] >= adjacents) == 0
}

# Data --------------------------------------------------------------------

heights <- readLines(here::here("2021/Day9/data.txt")) |>
  strsplit("", fixed = TRUE) |>
  map(as.integer) |>
  do.call(what = rbind)

# Part 1 ------------------------------------------------------------------

lowpoints_idx <- expand.grid(row = seq_len(nrow(heights)), col = seq_len(ncol(heights))) |>
  pmap_lgl(\(row, col) is_lowpoint(heights, row, col))

sum(heights[lowpoints_idx] + 1)

# Part 2 ------------------------------------------------------------------

basins <- ifelse(heights == 9, 0L, 1L)

basins |>
  rast() |>
  patches(zeroAsNA = TRUE) |>
  as.data.frame() |>
  count(lyr.1) |>
  arrange(-n) |>
  head(3) |>
  summarise(result = prod(n))
