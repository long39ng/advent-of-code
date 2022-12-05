# Utils -------------------------------------------------------------------

extract_longest <- function(xs) xs[[which.max(lengths(xs))]]

move_crates <- function(stacks, rearrangement, all_at_once = FALSE) {
  stopifnot(all(c("n", "from", "to") %in% names(rearrangement)))

  crates_to_move <- utils::tail(stacks[[rearrangement[["from"]]]], rearrangement[["n"]])
  if (!all_at_once) crates_to_move <- rev(crates_to_move)

  stacks[[rearrangement[["to"]]]] <- c(
    stacks[[rearrangement[["to"]]]],
    crates_to_move
  )
  stacks[[rearrangement[["from"]]]] <-
    utils::head(stacks[[rearrangement[["from"]]]], -rearrangement[["n"]])

  stacks
}

# Data --------------------------------------------------------------------

input <- readLines(here::here("2022/05-input"))
# input <- readLines(here::here("2022/inputu"))

input_section_lengths <- rle(nzchar(input))$lengths

stacks_strs <- input |>
  utils::head(input_section_lengths[1] - 1) |>
  rev()

crate_str_locs <- stacks_strs |>
  stringi::stri_locate_all_regex("[A-Z]") |>
  extract_longest()

stacks <- stacks_strs |>
  purrr::map(stringi::stri_sub, crate_str_locs) |>
  purrr::transpose() |>
  purrr::map(purrr::discard, \(x) !x %in% LETTERS) |>
  purrr::simplify_all()

procedure <- input |>
  utils::tail(input_section_lengths[3]) |>
  stringi::stri_extract_all_regex("\\d+") |>
  purrr::map(as.integer) |>
  purrr::map(purrr::set_names, c("n", "from", "to"))

# Part 1 ------------------------------------------------------------------

purrr::reduce(procedure, move_crates, .init = stacks) |>
  purrr::map(utils::tail, 1) |>
  paste0(collapse = "")

# Part 2 ------------------------------------------------------------------

purrr::reduce(procedure, move_crates, .init = stacks, all_at_once = TRUE) |>
  purrr::map(utils::tail, 1) |>
  paste0(collapse = "")
