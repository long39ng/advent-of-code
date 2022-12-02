input <- readLines(here::here("2022/02-input")) |>
  strsplit(" ", fixed = TRUE) |>
  purrr::map(purrr::set_names, c("a0", "b0")) |>
  dplyr::bind_rows()

rules <- tidyr::expand_grid(
  a = 1:3, # 1: rock, 2: paper, 3: scissors
  b = 1:3,
) |>
  dplyr::mutate(
    b_rs = (a - b + 3) %% 3, # 0: draw, 1: b loses, 2: b wins
    b_out = dplyr::case_when(
      b_rs == 0 ~ 3,
      b_rs == 1 ~ 0,
      b_rs == 2 ~ 6
    ),
    score = b + b_out
  )

# Part 1 ------------------------------------------------------------------

letters_values <- dplyr::tibble(
  letter = c("A", "B", "C", "X", "Y", "Z"),
  value = rep(1:3, 2)
)

input |>
  lo.ng::enrich_join(letters_values, a = value, by = c("a0" = "letter")) |>
  lo.ng::enrich_join(letters_values, b = value, by = c("b0" = "letter")) |>
  lo.ng::enrich_join(rules, score, by = c("a", "b")) |>
  dplyr::summarise(sum(score))

# Part 2 ------------------------------------------------------------------

letters_values2 <- dplyr::tibble(
  letter = c("A", "B", "C", "X", "Y", "Z"),
  value = c(1, 2, 3, 1, 0, 2)
)

input |>
  lo.ng::enrich_join(letters_values2, a = value, by = c("a0" = "letter")) |>
  lo.ng::enrich_join(letters_values2, b_rs = value, by = c("b0" = "letter")) |>
  lo.ng::enrich_join(rules, score, by = c("a", "b_rs")) |>
  dplyr::summarise(sum(score))
