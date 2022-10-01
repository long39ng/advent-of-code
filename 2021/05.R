# Day 5
library(dplyr)
library(purrr)
library(sf)

# Utils -------------------------------------------------------------------

prep_multipoint <- function(x, y) {
  n_points <- max(length(x), length(y))
  map2(x, y, c) |> unlist() |> matrix(n_points, 2, byrow = TRUE)
}

# Data --------------------------------------------------------------------

vents_df <- readr::read_delim(here::here("2021/05-input"),
  delim = " -> ",
  col_names = c("A", "B"),
  col_types = "cc"
) |>
  mutate(
    across(c(A, B), \(x) strsplit(x, ",", fixed = TRUE) |> map(as.numeric)),
    x = map2(A, B, \(a, b) a[1]:b[1]),
    y = map2(A, B, \(a, b) a[2]:b[2]),
    points = map2(x, y, prep_multipoint)
  )

# Part 1 ------------------------------------------------------------------

vents_df |>
  filter(map2_lgl(A, B, \(a, b) a[1] == b[1] | a[2] == b[2])) |>
  mutate(geometry = map(points, st_multipoint)) |>
  st_as_sf() |>
  st_intersection() |>
  filter(n.overlaps > 1) |>
  st_union() |>
  st_cast("POINT") |>
  length()

# Part 2 ------------------------------------------------------------------

vents_df |>
  mutate(geometry = map(points, st_multipoint)) |>
  st_as_sf() |>
  st_intersection() |>
  filter(n.overlaps > 1) |>
  st_union() |>
  st_cast("POINT") |>
  length()
