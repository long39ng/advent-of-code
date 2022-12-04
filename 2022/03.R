input <- readLines(here::here("2022/03-input")) |>
  strsplit("", fixed = TRUE)

letters_values <- dplyr::tibble(letter = c(letters, LETTERS)) |>
  dplyr::mutate(value = dplyr::row_number())

# Part 1 ------------------------------------------------------------------

input |>
  purrr::map(\(x) {
    item_seq <- 1:(length(x) / 2)
    list(x[item_seq], x[-item_seq])
  }) |>
  purrr::transpose() |>
  purrr::pmap_chr(intersect) |>
  dplyr::tibble(letter = _) |>
  dplyr::inner_join(letters_values) |>
  dplyr::summarise(sum(value))

# Part 2 ------------------------------------------------------------------

dplyr::tibble(rucksack = input) |>
  dplyr::mutate(group = (dplyr::row_number() - 1) %/% 3) |>
  dplyr::group_by(group) |>
  dplyr::summarise(letter = purrr::reduce(rucksack, intersect)) |>
  dplyr::inner_join(letters_values) |>
  dplyr::summarise(sum(value))
