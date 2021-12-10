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

str_mask_chunks <- function(str) {
  str |>
    stri_replace_all_fixed(ref$open, "0", vectorise_all = FALSE) |>
    stri_replace_all_fixed(ref$close, "1", vectorise_all = FALSE)
}

illegal_close <- function(str) {
  mask <- str_mask_chunks(str)
  chunk_mask <- "0v*1"

  while (stri_detect_regex(mask, chunk_mask)) {
    chunk_pos <- stri_locate_first_regex(mask, chunk_mask)[1,]
    open <- stri_sub(str, chunk_pos[[1]], length = 1)
    close <- stri_sub(str, chunk_pos[[2]], length = 1)

    if (ref$close[ref$open == open] != close) return(close)

    stri_sub(mask, chunk_pos[[1]], length = 1) <- "v"
    stri_sub(mask, chunk_pos[[2]], length = 1) <- "v"
  }
  NA_character_
}

need_autocomplete <- function(str) {
  mask <- str_mask_chunks(str)
  chunk_mask <- "0v*1"

  while (stri_detect_regex(mask, chunk_mask)) {
    chunk_pos <- stri_locate_first_regex(mask, chunk_mask)[1,]
    stri_sub(mask, chunk_pos[[1]], length = 1) <- "v"
    stri_sub(mask, chunk_pos[[2]], length = 1) <- "v"
  }
  open_locs <- stri_locate_all_fixed(mask, "0", get_length = TRUE)[[1]][,1]

  strsplit(str, "")[[1]][open_locs]
}

autocomplete_points <- function(str) {
  tibble(open = rev(need_autocomplete(str))) |>
    left_join(ref, by = "open") |>
    pull(points2) |>
    reduce(\(x, y) x * 5 + y)
}

# Data --------------------------------------------------------------------

input <- tibble(x = readLines(here::here("2021/Day10/data.txt")))

# Part 1 ------------------------------------------------------------------

input |>
  mutate(illegal = map_chr(x, illegal_close)) |>
  count(illegal) |>
  inner_join(ref, by = c("illegal" = "close")) |>
  summarise(result = sum(n * points1))

# Part 2 ------------------------------------------------------------------

input |>
  mutate(illegal = map_chr(x, illegal_close)) |>
  filter(is.na(illegal)) |>
  mutate(points = map_dbl(x, autocomplete_points)) |>
  summarise(result = median(points))
