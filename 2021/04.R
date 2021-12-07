# Day 4
library(dplyr)
library(purrr)

# Utils -------------------------------------------------------------------

when_complete_row_or_col <- function(results_by_round, which) {
  sum_fun <- switch(which, row = rowSums, col = colSums, stop())

  results_by_round |>
    map(sum_fun) |>
    keep(\(x) 5 %in% x) |>
    names() |>
    first()
}

when_complete <- function(results_by_round) {
  c("row", "col") |>
    map_chr(\(x) when_complete_row_or_col(results_by_round, which = x)) |>
    as.integer() |>
    min()
}

# Data --------------------------------------------------------------------

draws <- readLines(here::here("2021/Day4/data.txt"), n = 1) |>
  strsplit(",", fixed = TRUE) |>
  unlist() |>
  as.integer() %>%
  set_names(seq_along(.))

boards <- readr::read_fwf(here::here("2021/Day4/data.txt"), skip = 1) |>
  tidyr::drop_na() %>%
  mutate(id = rep(seq_len(nrow(.) / 5), each = 5)) |>
  tidyr::nest(board = -id) |>
  mutate(board = map(board, as.matrix))

# Part 1 ------------------------------------------------------------------

bingo_results <- boards |>
  mutate(marked = map(board, \(board) {
    accumulate(
      .init = matrix(rep(0, 25), 5, 5),
      draws,
      \(x, y) x + matrix(board %in% y, 5, 5)
    )[-1] # drop .init
  })) |>
  mutate(
    complete = map_int(marked, when_complete),
    marked_at_complete = map2(marked, complete, pluck)
  )

bingo_results |>
  slice_min(complete) |>
  mutate(result = pmap_dbl(
    list(board, complete, marked_at_complete),
    \(x, y, z) sum(x * draws[y] * (1 - z)))
  )

# Part 2 ------------------------------------------------------------------

bingo_results |>
  slice_max(complete) |>
  mutate(result = pmap_dbl(
    list(board, complete, marked_at_complete),
    \(x, y, z) sum(x * draws[y] * (1 - z)))
  )
