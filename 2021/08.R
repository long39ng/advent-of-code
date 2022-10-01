library(dplyr)
library(purrr)

# Utils -------------------------------------------------------------------

chardiffs <- function(x, y) {
  x <- unlist(strsplit(x, "", fixed = TRUE))
  y <- unlist(strsplit(y, "", fixed = TRUE))
  length(setdiff(x, y))
}

# Data --------------------------------------------------------------------

patterns <- readr::read_delim(
  here::here("2021/08-input"),
  delim = " | ",
  col_names = c("input", "output")
) |>
  mutate(
    across(input:output, strsplit, split = " ", fixed = TRUE),
    across(input:output, \(x) map(x, ~ as_tibble(.x) |>
      mutate(
        value = map(strsplit(value, "", fixed = TRUE), sort),
        value = map_chr(value, paste0, collapse = ""),
        segments = nchar(value)
      ))
    )
  )

nchar_unique <- tibble(
  digit =    c(1, 4, 7, 8),
  segments = c(2, 4, 3, 7)
)

# Part 1 ------------------------------------------------------------------

patterns$output |>
  bind_rows() |>
  left_join(nchar_unique, by = "segments") |>
  filter(!is.na(digit)) |>
  nrow()

# Part 2 ------------------------------------------------------------------

patterns |>
  mutate(
    input = map(input, \(x) {
      left_join(x, nchar_unique, by = "segments") |>
        mutate(
          # 6: 6 segment, 1 has 1 segment that 6 does not
          digit = if_else(
            is.na(digit) & segments == 6 &
              map_int(value, chardiffs, x = value[which(digit == 1)]) == 1,
            6, digit
          ),
          # 3: 5 segment, contains 1
          digit = if_else(
            is.na(digit) & segments == 5 &
              map_int(value, chardiffs, x = value[which(digit == 1)]) == 0,
            3, digit
          ),
          # 9: 6 segment, has 1 segment that 3 does not
          digit = if_else(
            is.na(digit) & segments == 6 &
              map_int(value, chardiffs, y = value[which(digit == 3)]) == 1,
            9, digit
          ),
          # 0: rest of the digits with 6 segments
          digit = if_else(is.na(digit) & segments == 6, 0, digit),
          # 5: contained in 9,
          digit = if_else(
            is.na(digit) &
              map_int(value, chardiffs, y = value[which(digit == 9)]) == 0,
            5, digit
          ),
          # 2: the rest
          digit = if_else(is.na(digit), 2, digit)
        ) |>
        select(value, digit)
    }),
    output = map2(output, input, left_join, by = "value"),
    number = map_dbl(output, \(x) as.numeric(paste0(x$digit, collapse = "")))
  ) |>
  summarise(result = sum(number))
