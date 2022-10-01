library(purrr)

# Utils -------------------------------------------------------------------

cost_fun <- function(x, y, constant_burn = TRUE) {
  n_steps <- abs(x - y)

  if (constant_burn) return(sum(n_steps))

  # https://www.mathsisfun.com/algebra/triangular-numbers.html
  sum(n_steps * (n_steps + 1) / 2)
}

min_fuels <- function(x, constant_burn = TRUE) {
  positions <- min(x):max(x)

  fuels <- positions |>
    set_names() |>
    map_dbl(cost_fun, x, constant_burn)

  sort(fuels)[1]
}

# Data --------------------------------------------------------------------

crabs <- readLines(here::here("2021/07-input")) |>
  strsplit(",", fixed = TRUE) |>
  unlist() |>
  as.numeric()

# Part 1 ------------------------------------------------------------------

min_fuels(crabs)


# Part 2 ------------------------------------------------------------------

min_fuels(crabs, constant_burn = FALSE)
