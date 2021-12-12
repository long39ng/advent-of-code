library(dplyr)
library(purrr)

# Utils -------------------------------------------------------------------

adjacents <- function(m, row, col) {
  expand.grid(row = -1:1 + row, col = -1:1 + col) |>
    filter(
      between(row, 1, nrow(m)), between(col, 1, ncol(m)),
      (row != !!row | col != !!col)
    )
}

flash <- function(m, row, col) {
  n_flashed <<- n_flashed + 1

  # Each octopus can only flash once:
  m[row, col] <- -999

  to_increase <- adjacents(m, row, col)

  pwalk(to_increase, \(...) {
    m[..1, ..2] <<- m[..1, ..2] + 1
  })
  m
}

flash_matrix <- function(m) {
  expand.grid(row = seq_len(nrow(m)), col = seq_len(ncol(m))) |>
    pwalk(\(row, col) {
      if (m[row, col] > 9) m <<- flash(m, row, col)
    })
  if (sum(m > 9) == 0) {
    m[m < 0] <- 0
    return(m)
  }
  flash_matrix(m)
}

# Data --------------------------------------------------------------------

input <- readLines(here::here("2021/Day11/data.txt")) |>
  strsplit("", fixed = TRUE) |>
  map(as.numeric) |>
  do.call(what = rbind)

# Part 1 ------------------------------------------------------------------

state <- input
n_flashed <- 0

for (step in 1:100) state <- flash_matrix(state + 1)

n_flashed

# Part 2 ------------------------------------------------------------------

state <- input
step <- 0

repeat {
  step <- step + 1
  state <- flash_matrix(state + 1)
  if (sum(state > 0) == 0) break
}

step
