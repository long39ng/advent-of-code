input <- dplyr::tibble(cal = as.integer(readLines(here::here("2022/01-input")))) |>
  dplyr::mutate(reeindeer = cumsum(is.na(cal))) |>
  tidyr::drop_na(cal)

cal_counts <- input |>
  dplyr::count(reeindeer, wt = cal, sort = TRUE)

# Part 1 ------------------------------------------------------------------

cal_counts |>
  dplyr::slice(1)

# Part 2 ------------------------------------------------------------------

cal_counts |>
  dplyr::slice(1:3) |>
  dplyr::summarise(sum(n))
