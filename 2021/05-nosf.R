# Day 5
library(dplyr)
library(tidyr)
library(purrr)

# Data --------------------------------------------------------------------

vents <- readr::read_lines(here::here("2021/Day5/data.txt")) |>
  as_tibble() |>
  separate(value, into = c("x1", "y1", "x2", "y2"), convert = TRUE) |>
  mutate(id = row_number()) |>
  nest(data = -id)

# Part 1 ------------------------------------------------------------------

vents |>
  filter(map_lgl(data, \(df) df$x1 == df$x2 | df$y1 == df$y2)) |>
  transmute(all_points = map(data, \(df) tibble(x = df$x1:df$x2, y = df$y1:df$y2))) |>
  unnest(all_points) |>
  count(x, y) |>
  filter(n > 1) |>
  nrow()

# Part 2 ------------------------------------------------------------------

vents |>
  transmute(all_points = map(data, \(df) tibble(x = df$x1:df$x2, y = df$y1:df$y2))) |>
  unnest(all_points) |>
  count(x, y) |>
  filter(n > 1) |>
  nrow()
