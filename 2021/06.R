# Day 6
library(dplyr)
library(purrr)

# Data --------------------------------------------------------------------

school_init <- readLines(here::here("2021/Day6/data.txt")) |>
  strsplit(",", fixed = TRUE) |>
  unlist() |>
  as.numeric() |>
  as_tibble()

# Part 1 ------------------------------------------------------------------

counts_init <- school_init |>
  count(value) |>
  add_row(value = c(0, 6:8), n = 0) |>
  arrange(value)

reduce(
  .init = counts_init,
  1:80,
  ~ .x |>
    mutate(n = case_when(
      value == 6 ~ lead(n) + first(n),
      value == 8 ~ first(n),
      TRUE ~ lead(n)
    ))
) |>
  summarise(result = sum(n))

# Part 2 ------------------------------------------------------------------

reduce(
  .init = counts_init,
  1:256,
  ~ .x |>
    mutate(n = case_when(
      value == 6 ~ lead(n) + first(n),
      value == 8 ~ first(n),
      TRUE ~ lead(n)
    ))
) |>
  summarise(result = sum(n))
