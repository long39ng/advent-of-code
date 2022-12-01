library(tidyverse)

input <- tibble(cal = as.integer(readLines(here::here("2022/01-input")))) |>
  mutate(reeindeer = data.table::rleid(is.na(cal))) |>
  drop_na(cal)

cal_counts <- input |>
  count(reeindeer, wt = cal)

# Part 1 ------------------------------------------------------------------

cal_counts |>
  slice_max(n, n = 1)

# Part 2 ------------------------------------------------------------------

cal_counts |>
  slice_max(n, n = 3) |>
  summarise(sum(n))
