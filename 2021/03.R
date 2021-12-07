# Day 3
library(dplyr)
library(purrr)

# Utils -------------------------------------------------------------------

binary_mode <- function(x, ties = 1) {
  m <- mean(x, na.rm = TRUE)
  if (m == .5) return(ties)
  round(m)
}

rev_binary <- function(x) {
  bits <- strsplit(x, "", fixed = TRUE) |> unlist() |> as.integer()
  paste0(1 - bits, collapse = "")
}

keep_mode_by_col <- function(tbl, rev = FALSE) {
  compare <- if (rev) `!=` else `==`

  # Functional programming ftw!
  accumulate(
    .init = tbl,
    names(tbl),
    possibly(
      \(x, y) filter(x, compare(.data[[y]], binary_mode(.data[[y]]))),
      otherwise = tibble()
    )
  )
}

decimal_from_last_row <- function(list) {
  list |>
    keep(\(x) nrow(x) == 1) |>
    pluck(1) |>
    pmap_chr(paste0) |>
    strtoi(base = 2)
}

# Data --------------------------------------------------------------------

input_df <- readLines(here::here("2021/Day3/data.txt")) |>
  strsplit("", fixed = TRUE) |>
  map(\(x) set_names(as.numeric(x), paste0("bit", seq_along(x)))) |>
  bind_rows()

# Part 1 ------------------------------------------------------------------

input_df |>
  summarise(across(everything(), binary_mode)) |>
  pmap_dfc(paste0) |>
  set_names("gamma") |>
  mutate(
    epsilon = rev_binary(gamma),
    across(everything(), strtoi, base = 2),
    power_consumption = gamma * epsilon
  )

# Part 2 ------------------------------------------------------------------

c(o2 = FALSE, co2 = TRUE) |>
  map_dfc(\(x) {
    keep_mode_by_col(input_df, rev = x) |>
      decimal_from_last_row()
  }) |>
  mutate(life_support = o2 * co2)
