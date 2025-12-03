= Day 2: Gift Shop

The ranges are separated by commas (`,`); each range gives its *first ID* and *last ID* separated by a dash (`-`).

Since the young Elf was just doing silly patterns, you can find the *invalid IDs* by looking for any ID which is made only of some sequence of digits repeated twice. So, `55` (`5` twice), `6464` (`64` twice), and `123123` (`123` twice) would all be invalid IDs.

None of the numbers have leading zeroes; `0101` isn't an ID at all. (`101` is a *valid* ID that you would ignore.)

What do you get if you add up all of the invalid IDs?

#let id-ranges = read("02-input.txt").trim().split(",").map(s => s.split("-"))

Answer:
#{
  let is-invalid(id) = {
    let id-str = str(id)
    let digits = id-str.len()
    (
      calc.rem(digits, 2) == 0
        and {
          let half = calc.quo(digits, 2)
          id-str.slice(0, half) == id-str.slice(half)
        }
    )
  }
  id-ranges.map(((start, end)) => range(int(start), int(end) + 1)).flatten().filter(is-invalid).sum()
}

== Part Two

Now, an ID is invalid if it is made only of some sequence of digits repeated *at least* twice. So, `12341234` (`1234` two times), `123123123` (`123` three times), `1212121212` (`12` five times), and `1111111` (`1` seven times) are all invalid IDs.

What do you get if you add up all of the invalid IDs using these new rules?

Answer:
#{
  let is-invalid(id) = {
    let id-str = str(id)
    // A string contains repeated sequences if it is found within itself twice, minus the first and last characters
    let id-twice = str(id-str * 2)
    id-twice.len() > 2 and id-twice.slice(1, id-twice.len() - 1).contains(id-str)
  }
  id-ranges.map(((start, end)) => range(int(start), int(end) + 1)).flatten().filter(is-invalid).sum()
}
