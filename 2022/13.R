is_right_order <- function(l, r) {
  equals <- which_equal(l, r)
  if (length(equals)) {
    l <- l[-equals]
    r <- r[-equals]
  }

  if (!length(r)) {
    FALSE
  } else if (!length(l)) {
    TRUE
  } else if (is.atomic(l) && is.atomic(r)) {
    l < r
  } else if (is.atomic(l) && is.recursive(r)) {
    is_right_order(list(l), r)
  } else if (is.recursive(l) && is.atomic(r)) {
    is_right_order(l, list(r))
  } else {
    is_right_order(l[[1]], r[[1]])
  }
}

which_equal <- function(l, r) {
  idx <- seq_len(min(length(l), length(r)))
  which(purrr::map2_lgl(l[idx], r[idx], \(x, y) identical(unlist(x), unlist(y))))
}

input <- readLines(here::here("2022/13-input"))

pairs <- input |>
  split(cumsum(!nzchar(input))) |>
  purrr::map(purrr::keep, nzchar) |>
  purrr::map_depth(2, jsonlite::fromJSON, simplifyVector = FALSE) |>
  unname()

# Part 1 ------------------------------------------------------------------

pairs |>
  purrr::map_lgl(purrr::lift_dl(is_right_order)) |>
  which() |>
  sum()

# Part 2 ------------------------------------------------------------------

n_less_than <- function(xs, y) sum(purrr::map_lgl(xs, is_right_order, y))

div1 <- list(list(2L))
div2 <- list(list(6L))

packets <- purrr::flatten(pairs)

(n_less_than(packets, div1) + 1) * (n_less_than(packets, div2) + 2)
