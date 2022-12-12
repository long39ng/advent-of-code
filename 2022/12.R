adjacent_nodes <- function(idx, dims) {
  arr_idx <- arrayInd(idx, dims)

  expand.grid(i = -1:1 + arr_idx[1], j = -1:1 + arr_idx[2]) |>
    dplyr::filter(
      xor(i == arr_idx[1], j == arr_idx[2]),
      dplyr::between(i, 1, dims[1]),
      dplyr::between(j, 1, dims[2])
    ) |>
    purrr::pmap_int(\(i, j) (j - 1L) * dims[1] + i)
}

# Data --------------------------------------------------------------------

input <- readLines("2022/12-input") |>
  strsplit("", fixed = TRUE) |>
  do.call(what = rbind)

start_node <- which(input == "S")
end_node <- which(input == "E")

heights <- input |>
  sub(x = _, "S", "a") |>
  sub(x = _, "E", "z") |>
  purrr::map_int(\(x) which(letters == x)) |>
  matrix(nrow = nrow(input))

graph <- data.frame(from = seq_along(heights)) |>
  dplyr::mutate(to = purrr::map(from, \(x) {
    adj <- adjacent_nodes(x, dim(heights))
    adj[heights[x] >= heights[adj] - 1L]
  })) |>
  tidyr::unnest(to) |>
  igraph::graph_from_data_frame()

# Part 1 ------------------------------------------------------------------

igraph::distances(graph, start_node, end_node, mode = "out")

# Part 2 ------------------------------------------------------------------

which(heights == 1) |>
  igraph::distances(graph, v = _, end_node, mode = "out") |>
  min()
