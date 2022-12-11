# Data --------------------------------------------------------------------

parse_monkey <- function(lines) {
  dplyr::lst(
    items = lines[2] |>
      sub(pattern = "  Starting items: ", replacement = "", fixed = TRUE) |>
      strsplit(", ", fixed = TRUE) |>
      unlist() |>
      as.numeric(),
    op = lines[3] |>
      sub(pattern = "  Operation: new =", replacement = "\\(old)", fixed = TRUE) |>
      str2lang() |>
      eval(),
    test_div = readr::parse_number(lines[4]),
    throw = c(test_div, readr::parse_number(lines[5:6])) |>
      as.list() |>
      purrr::pmap_chr(sprintf, fmt = "\\(x) if (x %%%% %s == 0) '%s' else '%s'") |>
      str2lang() |>
      eval(),
    n_inspections = 0
  )
}

input <- readLines(here::here("2022/11-input"))

monkeys <- input |>
  split(cumsum(nchar(input) == 0)) |>
  purrr::map(purrr::keep, nzchar) |>
  purrr::map(parse_monkey)

modulo <- monkeys |>
  purrr::map_dbl("test_div") |>
  prod()

# Solution ----------------------------------------------------------------

monkey_business <- function(monkeys, n_rounds, relief = TRUE) {
  seq_len(n_rounds) |>
    purrr::reduce(simulate_round, .init = monkeys, relief = relief) |>
    purrr::map_dbl("n_inspections") |>
    sort(decreasing = TRUE) |>
    utils::head(2) |>
    prod()
}

simulate_round <- function(monkeys, .round_nr = integer(1), relief) {
  names(monkeys) |>
    purrr::reduce(simulate_monkey, .init = monkeys, relief = relief)
}

simulate_monkey <- function(monkeys, name, relief) {
  rep_len(name, length(monkeys[[name]]$items)) |>
    purrr::reduce(inspect_item, .init = monkeys, relief = relief)
}

inspect_item <- function(monkeys, name, relief) {
  monkeys[[name]]$n_inspections <- monkeys[[name]]$n_inspections + 1

  worry <- monkeys[[name]]$op(monkeys[[name]]$items[1])
  worry <- if (relief) worry %/% 3 else worry %% modulo

  monkeys[[name]]$items <- monkeys[[name]]$items[-1]

  recipient <- monkeys[[name]]$throw(worry)
  monkeys[[recipient]]$items <- c(monkeys[[recipient]]$items, worry)

  monkeys
}

# Part 1 ------------------------------------------------------------------

monkey_business(monkeys, 20)

# Part 2 ------------------------------------------------------------------

monkey_business(monkeys, 10000, relief = FALSE)
