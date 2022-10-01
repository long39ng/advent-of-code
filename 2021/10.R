library(dplyr)
library(stringi)
library(purrr)

# Utils -------------------------------------------------------------------

ref <- tribble(
  ~open, ~close, ~points1, ~points2,
  "(",   ")",           3,        1,
  "[",   "]",          57,        2,
  "{",   "}",        1197,        3,
  "<",   ">",       25137,        4
)

chunk_regex <- "\\(\\)|\\[\\]|\\{\\}|\\<\\>"
close_regex <- "\\)|\\]|\\}|\\>"

remove_closed_chunks <- function(str) {
  while (stri_detect_regex(str, chunk_regex)) {
    str <- stri_replace_all_regex(str, chunk_regex, "")
  }
  str
}

first_illegal_close <- function(str) {
  str <- remove_closed_chunks(str)
  stri_extract_first_regex(str, close_regex)
}

need_autocomplete <- function(str) {
  str <- remove_closed_chunks(str)
  rev(strsplit(str, "")[[1]])
}

autocomplete_points <- function(str) {
  tibble(open = need_autocomplete(str)) |>
    left_join(ref, by = "open") |>
    pull(points2) |>
    reduce(\(x, y) x * 5 + y)
}

# Data --------------------------------------------------------------------

input <- tibble(x = readLines(here::here("2021/10-input")))

# Part 1 ------------------------------------------------------------------

input |>
  mutate(illegal = map_chr(x, first_illegal_close)) |>
  count(illegal) |>
  inner_join(ref, by = c("illegal" = "close")) |>
  summarise(result = sum(n * points1))

# Part 2 ------------------------------------------------------------------

input |>
  mutate(illegal = map_chr(x, first_illegal_close)) |>
  filter(is.na(illegal)) |>
  mutate(points = map_dbl(x, autocomplete_points)) |>
  summarise(result = median(points))
