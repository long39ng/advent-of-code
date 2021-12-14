library(stringi)
library(dplyr)
library(purrr)

# Utils -------------------------------------------------------------------

split_by_insertion <- function(pair, insert) {
  stopifnot(stri_length(pair) == 2 && stri_length(insert) == 1)
  c(
    paste0(stri_sub(pair, 1, length = 1), insert),
    paste0(insert, stri_sub(pair, 2, length = 1))
  )
}

count_pairs <- function(polymer) {
  tibble(pair = map_chr(
    seq_len(stri_length(polymer) - 1),
    \(x) stri_sub(polymer, from = x, length = 2)
  )) |>
    count(pair)
}

count_pairs_after_steps <- function(polymer, steps) {
  reduce(
    .init = count_pairs(polymer),
    seq_len(steps),
    ~ .x |>
      left_join(rules, by = "pair") |>
      tidyr::unnest(split_into) |>
      count(split_into, wt = n) |>
      rename(pair = split_into)
  )
}

count_elements_after_steps <- function(polymer, steps) {
  count_pairs_after_steps(polymer, steps) |>
    mutate(element = stri_sub(pair, 1, 1)) |>
    count(element, wt = n) |>
    # The last element never appears as the first character in a pair:
    mutate(n = if_else(element == stri_sub(polymer, -1), n + 1L, n))
}

# Data --------------------------------------------------------------------

template <- readLines(here::here("2021/Day14/data.txt"), n = 1)

rules <- readr::read_delim(
  here::here("2021/Day14/data.txt"),
  delim = " -> ",
  skip = 1,
  col_names = c("pair", "insert")
) |>
  mutate(split_into = map2(pair, insert, split_by_insertion))

# Part 1 ------------------------------------------------------------------

count_elements_after_steps(template, 10) |>
  summarise(result = max(n) - min(n))

# Part 2 ------------------------------------------------------------------

count_elements_after_steps(template, 40) |>
  summarise(result = max(n) - min(n))
