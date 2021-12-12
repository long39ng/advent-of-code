library(dplyr)
library(purrr)

# Utils -------------------------------------------------------------------

dfs_recursive <- function(graph, root, avoid = root, small_visits = 1) {
  # End search if "end" is reached
  if (root == "end") return(1)

  paths <- 0
  walk(graph$to[graph$from == root], \(x) {
    if (!x %in% avoid) {
      if (grepl("^[a-z]{2}$", x)) {
        # Avoid visiting small caves again
        paths <<- paths + dfs_recursive(graph, x, c(avoid, x), small_visits)
      } else {
        paths <<- paths + dfs_recursive(graph, x, avoid, small_visits)
      }
    } else if (small_visits == 2) {
      # Don't add small caves to avoid list after the first visit
      paths <<- paths + dfs_recursive(graph, x, avoid, small_visits = 1)
    }
  })
  paths
}

# Data --------------------------------------------------------------------

input <- readr::read_delim(here::here("2021/Day12/data.txt"), delim = "-", col_names = FALSE)

cave_net <- bind_rows(
  input |> rename(from = X1, to = X2),
  input |> rename(from = X2, to = X1)
) |>
  # "start" cannot be visited again
  filter(to != "start")

# Part 1 ------------------------------------------------------------------

dfs_recursive(cave_net, "start", small_visits = 1)

# Part 2 ------------------------------------------------------------------

dfs_recursive(cave_net, "start", small_visits = 2)
