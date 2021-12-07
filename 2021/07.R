library(purrr)

# Utils -------------------------------------------------------------------

cost_fun <- function(x, y, constant_burn = TRUE) {
  n_steps <- abs(x - y)

  if (constant_burn) return(sum(n_steps))

  sum(map_dbl(n_steps, \(z) seq_len(z) |> c() |> sum()))
}

min_fuels <- function(x, constant_burn = TRUE) {
  positions <- min(x):max(x)

  fuels <- positions |>
    set_names() |>
    map_dbl(cost_fun, x, constant_burn)

  sort(fuels)[1]
}

# Data --------------------------------------------------------------------

crabs <- readLines(here::here("2021/Day7/data.txt")) |>
  strsplit(",", fixed = TRUE) |>
  unlist() |>
  as.numeric()

# Part 1 ------------------------------------------------------------------

min_fuels(crabs)


# Part 2 ------------------------------------------------------------------

min_fuels(crabs, constant_burn = FALSE)
