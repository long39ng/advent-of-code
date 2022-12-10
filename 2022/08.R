input <- readLines(here::here("2022/08-input")) |>
  strsplit("", fixed = TRUE) |>
  purrr::map(as.integer) |>
  do.call(what = "rbind")

tree_locs <- expand.grid(
  i = seq_len(nrow(input)),
  j = seq_len(ncol(input))
)

is_visible <- function(mat, i, j) {
  lines_to_edges(mat, i, j) |>
    purrr::map_lgl(\(x) n_smaller(x, mat[i, j]) == length(x)) |>
    any()
}

scenic_score <- function(mat, i, j) {
  lines_to_edges(mat, i, j) |>
    purrr::map_int(viewing_distance, mat[i, j]) |>
    prod()
}

viewing_distance <- function(xs, y) min(n_smaller(xs, y) + 1L, length(xs))

n_smaller <- function(xs, y) length(purrr::head_while(xs, \(x) x < y))

lines_to_edges <- function(mat, i, j) {
  list(
    w = rev(mat[i, 1:j]),
    e = mat[i, j:dim(mat)[2]],
    n = rev(mat[1:i, j]),
    s = mat[i:dim(mat)[1], j]
  ) |>
    purrr::map(utils::tail, -1)
}

# Part 1 ------------------------------------------------------------------

tree_locs |>
  purrr::pmap_lgl(is_visible, mat = input) |>
  sum()

# Part 2 ------------------------------------------------------------------

tree_locs |>
  purrr::pmap_dbl(scenic_score, mat = input) |>
  max()
