= Day 5: Cafeteria

The database operates on *ingredient IDs*. It consists of a list of *fresh ingredient ID ranges*, a blank line, and a list of *available ingredient IDs*.

The fresh ID ranges are *inclusive*: the range `3-5` means that ingredient IDs `3`, `4`, and `5` are all *fresh*. The ranges can also *overlap*; an ingredient ID is fresh if it is in *any* range.

How many of the available ingredient IDs are fresh?

#let (id-ranges, available-ids) = read("05-input.txt").trim().split("\n\n").map(s => s.split("\n"))
#let id-ranges = id-ranges.map(s => s.split("-").map(int))
#let available-ids = available-ids.map(int)

Answer:
#{
  available-ids.filter(id => id-ranges.any(((start, end)) => start <= id and id <= end)).len()
}

== Part Two

So that they can stop bugging you when they get new inventory, the Elves would like to know *all* of the IDs that the *fresh ingredient ID ranges* consider to be *fresh*.

How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges?

Answer:
#{
  id-ranges
    .sorted()
    .fold((0, 0), ((acc, curr), (start, end)) => {
      let count-from = calc.max(start, curr + 1)
      let count-to = calc.max(end, curr)
      (acc + (count-to - count-from + 1), count-to)
    })
    .at(0)
}
