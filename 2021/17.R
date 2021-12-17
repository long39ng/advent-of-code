library(dplyr)
library(purrr)

# Utils -------------------------------------------------------------------

prob_position <- function(vx, vy, x = 0, y = 0, max_y = 0) {
  max_y <- max(y, max_y)
  # Miss:
  if (x > target$x2 || y < target$y1) {
    return(tibble::lst(max_y, hit = 0))
  }
  # Hit:
  if (between(x, target$x1, target$x2) && between(y, target$y1, target$y2)) {
    return(tibble::lst(max_y, hit = 1))
  }
  # Recurse with new position and velocity
  prob_position(vx - sign(vx), vy - 1, x + vx, y + vy, max_y)
}

# Data --------------------------------------------------------------------

target <- tibble(input = readLines(here::here("2021/Day17/data.txt"))) |>
  transmute(
    area = stringr::str_extract_all(input, "-?\\d+"),
    area = map(area, as.numeric),
    area = map(area, set_names, c("x1", "x2", "y1", "y2"))
  ) |>
  tidyr::unnest_wider(area)

# Part 1 ------------------------------------------------------------------

expand.grid(vx = 1:(target$x2 + 1), vy = -target$y1:target$y1) |>
  pmap_dfr(\(vx, vy) prob_position(vx, vy)) |>
  filter(hit == 1) |>
  slice_max(max_y)

# Part 2 ------------------------------------------------------------------

expand.grid(vx = 1:(target$x2 + 1), vy = -target$y1:target$y1) |>
  pmap_dfr(\(vx, vy) prob_position(vx, vy)) |>
  filter(hit == 1) |>
  nrow()
