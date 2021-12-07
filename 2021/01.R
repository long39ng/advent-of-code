# Day1
library(dplyr)

# Data --------------------------------------------------------------------

input_df <- readLines(here::here("2021/Day1/data.txt")) |>
  as.integer() |>
  as_tibble()

# Part 1 ------------------------------------------------------------------

input_df |>
  mutate(increased = value > lag(value)) |>
  summarise(n_increased = sum(increased, na.rm = TRUE))

# Part 2 ------------------------------------------------------------------

input_df |>
  mutate(increased = value > lag(value, n = 3)) |>
  summarise(n_increased = sum(increased, na.rm = TRUE))
