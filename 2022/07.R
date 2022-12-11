process_line <- function(acc, line) {
  cur_path <- acc$cur_path
  dir_sizes <- acc$dir_sizes

  if (line == "$ cd ..") {
    cur_path <- cur_path[-length(cur_path)]
  } else if (startsWith(line, "$ cd")) {
    cur_path <- c(cur_path, sub("$ cd ", "", line, fixed = TRUE))
  } else if (grepl("^\\d+", line, perl = TRUE)) {
    added_size <- as.numeric(strsplit(line, " ", fixed = TRUE)[[1]][1])

    upserted_dirs <- data.frame(dir = purrr::accumulate(cur_path, paste, sep = ".")) |>
      dplyr::left_join(dir_sizes, by = "dir") |>
      tidyr::replace_na(list(size = 0)) |>
      dplyr::mutate(size = size + added_size)

    dir_sizes <- dir_sizes |>
      dplyr::rows_upsert(upserted_dirs, by = "dir")
  }

  list(cur_path = cur_path, dir_sizes = dir_sizes)
}

input <- readLines(here::here("2022/07-input"))

acc <- purrr::reduce(
  input,
  process_line,
  .init = list(
    dir_sizes = data.frame(dir = character(), size = numeric()),
    cur_path = character()
  )
)

# Part 1 ------------------------------------------------------------------

acc$dir_sizes |>
  dplyr::filter(size <= 1e5) |>
  dplyr::summarise(sum(size))

# Part 2 ------------------------------------------------------------------

acc$dir_sizes |>
  dplyr::filter(size >= 3e7 - (7e7 - max(size))) |>
  dplyr::summarise(min(size))
