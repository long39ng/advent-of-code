which_first_marker <- function(chars, marker_length) {
  chars |>
    slider::slide(identity, .before = marker_length - 1) |>
    sapply(\(x) length(x) == marker_length && identical(x, unique(x))) |>
    which() |>
    min()
}

input <- readLines(here::here("2022/06-input")) |>
  strsplit("", fixed = TRUE) |>
  unlist()

# Part 1 ------------------------------------------------------------------

which_first_marker(input, marker_length = 4)

# Part 2 ------------------------------------------------------------------

which_first_marker(input, marker_length = 14)
