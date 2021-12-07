# Day 2
library(dplyr)

# Data --------------------------------------------------------------------

input_df <- readr::read_delim(
  here::here("2021/Day2/data.txt"),
  delim = " ",
  col_types = "ci",
  col_names = c("command", "value")
)

# Part 1 ------------------------------------------------------------------

input_df |>
  group_by(command) |>
  summarise(value = sum(value)) |>
  tidyr::pivot_wider(names_from = command, values_from = value) |>
  transmute(result = forward * (down - up))

# Part 2 ------------------------------------------------------------------

input_df |>
  mutate(
    aim_mod = case_when(
      command == "down" ~ 1L,
      command == "up" ~ -1L,
      TRUE ~ 0L),
    aim = cumsum(aim_mod * value),
    horizontal = (command == "forward") * value,
    depth = (command == "forward") * aim * value
  ) |>
  summarise(across(c(horizontal, depth), sum)) |>
  mutate(result = horizontal * depth)
