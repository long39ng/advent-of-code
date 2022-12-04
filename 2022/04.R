input <- readr::read_csv(
  here::here("2022/04-input"),
  col_names = c("a", "b")
) |>
  dplyr::mutate(dplyr::across(c(a, b), \(x) {
    x |>
      gsub(pattern = "-", replacement = ":", fixed = TRUE) |>
      purrr::map(\(x) eval(str2lang(x)))
  }))

# Part 1 ------------------------------------------------------------------

input |>
  purrr::pmap_lgl(\(a, b) {
    both <- union(a, b)
    length(both) == length(a) || length(both) == length(b)
  }) |>
  sum()

# Part 2 ------------------------------------------------------------------

input |>
  purrr::pmap_lgl(\(a, b) length(intersect(a, b)) != 0) |>
  sum()
