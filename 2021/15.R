library(igraph)
library(dplyr)
library(purrr)

# Utils -------------------------------------------------------------------

weighted_lattice <- function(m) {
  l <- make_lattice(dim(m), directed = TRUE, mutual = TRUE)

  weights <- get.edgelist(l) |>
    `colnames<-`(c("tail", "head")) |>
    as_tibble() |>
    mutate(weight = m[head]) |>
    pull(weight)

  l |> set_edge_attr("weight", value = weights)
}

min_total_risk <- function(m) {
  weighted_lattice(m) |>
    distances(v = 1, to = length(m), mode = "out")
}

# Data --------------------------------------------------------------------

input <- readLines(here::here("2021/Day15/data.txt")) |>
  strsplit("", fixed = TRUE) |>
  map(as.integer) |>
  do.call(what = rbind)

# Part 1 ------------------------------------------------------------------

min_total_risk(input)

# Part 2 ------------------------------------------------------------------

input |>
  # Expand horizontally
  list() |>
  map2(-1:3, \(m, i) (m + i) %% 9 + 1) |>
  do.call(what = cbind) |>
  # Expand vertically
  list() |>
  map2(-1:3, \(m, j) (m + j) %% 9 + 1) |>
  do.call(what = rbind) |>
  min_total_risk()
