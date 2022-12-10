input <- data.frame(raw = readLines(here::here("2022/10-input"))) |>
  tidyr::separate(raw, c("instruction", "value"), sep = " ", convert = TRUE) |>
  tidyr::replace_na(list(value = 0)) |>
  dplyr::mutate(
    cycle = cumsum((instruction == "addx") + 1) + 1,
    x = cumsum(value) + 1
  )

full_cycles <- data.frame(cycle = tidyr::full_seq(input$cycle, 1)) |>
  dplyr::left_join(input, by = "cycle") |>
  tidyr::fill(x, .direction = "down") |>
  dplyr::select(cycle, x) |>
  . => dplyr::bind_rows(data.frame(cycle = c(1, 2), x = 1), .)

# Part 1 ------------------------------------------------------------------

full_cycles |>
  dplyr::filter(cycle %in% seq(20, 220, 40)) |>
  dplyr::summarise(sum(x * cycle))

# Part 2 ------------------------------------------------------------------

full_cycles |>
  dplyr::mutate(
    row = (cycle - 1) %/% 40,
    crt_pos = cycle %% 40 - 1,
    sprite = purrr::map(x, \(x) x + -1:1),
    pixel = purrr::map2_chr(crt_pos, sprite, \(x, y) dplyr::if_else(x %in% y, "#", "."))
  ) |>
  dplyr::group_split(row) |>
  purrr::walk(\(x) cat(x$pixel, "\n"))
