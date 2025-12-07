= Day 4: Printing Department

The forklifts can only access a roll of paper if there are *fewer than four rolls of paper* in the eight adjacent positions.

How many rolls of paper can be accessed by a forklift?

#let diagram = read("04-input.txt").trim().split("\n").map(s => s.clusters())

#let (nrow, ncol) = (diagram.len(), diagram.at(0).len())

#let roll-locations = (
  diagram
    .enumerate()
    .map(((i, row)) => row.enumerate().filter(((_, x)) => x == "@").map(((j, _)) => nrow * i + j))
    .flatten()
)

#let nb-locations(loc) = {
  let (i, j) = (calc.quo(loc, nrow), calc.rem(loc, ncol))
  let nbs = ()

  for di in (-1, 0, 1) {
    for dj in (-1, 0, 1) {
      if di == 0 and dj == 0 { continue }

      let (nb-i, nb-j) = (i + di, j + dj)

      if nb-i < 0 or nb-i >= nrow or nb-j < 0 or nb-j >= ncol { continue }

      nbs.push(nrow * nb-i + nb-j)
    }
  }

  nbs
}

#let is-accessible(loc, roll-locs) = {
  nb-locations(loc).filter(nb => nb in roll-locs).len() < 4
}

Answer:
#{
  roll-locations.filter(loc => is-accessible(loc, roll-locations)).len()
}

== Part Two

Once a roll of paper can be accessed by a forklift, it can be *removed*. Once a roll of paper is removed, the forklifts might be able to access *more* rolls of paper, which they might also be able to remove.

Stop once no more rolls of paper are accessible by a forklift.

How many rolls of paper in total can be removed by the Elves and their forklifts?

Answer:
#{
  let sum-rolls = 0

  while true {
    let accessible-rolls = roll-locations.filter(loc => is-accessible(loc, roll-locations))

    if accessible-rolls.len() == 0 {
      break
    }

    sum-rolls += accessible-rolls.len()
    roll-locations = roll-locations.filter(loc => loc not in accessible-rolls)
  }

  sum-rolls
}
