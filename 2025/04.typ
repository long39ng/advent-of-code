= Day 4: Printing Department

The forklifts can only access a roll of paper if there are *fewer than four rolls of paper* in the eight adjacent positions.

How many rolls of paper can be accessed by a forklift?

#let diagram = read("04-input.txt").trim().split("\n").map(s => s.clusters())

#let (nrow, ncol) = (diagram.len(), diagram.at(0).len())

#let queen-nbs(mat, i, j) = {
  let nbs = ()

  for di in (-1, 0, 1) {
    for dj in (-1, 0, 1) {
      if di == 0 and dj == 0 { continue }

      let (nb-i, nb-j) = (i + di, j + dj)

      if nb-i < 0 or nb-i >= nrow or nb-j < 0 or nb-j >= ncol { continue }

      nbs.push(mat.at(nb-i).at(nb-j))
    }
  }

  nbs
}

#let is-accessible-roll(mat, i, j) = {
  mat.at(i).at(j) == "@" and queen-nbs(mat, i, j).filter(x => x == "@").len() < 4
}

Answer:
#{
  range(nrow * ncol)
    .filter(idx => {
      is-accessible-roll(diagram, calc.quo(idx, nrow), calc.rem(idx, ncol))
    })
    .len()
}

== Part Two

Once a roll of paper can be accessed by a forklift, it can be *removed*. Once a roll of paper is removed, the forklifts might be able to access *more* rolls of paper, which they might also be able to remove.

Stop once no more rolls of paper are accessible by a forklift.

How many rolls of paper in total can be removed by the Elves and their forklifts?

Answer:
#{
  let total-rolls = 0

  while true {
    let accessible-rolls = range(nrow * ncol)
      .map(idx => (calc.quo(idx, nrow), calc.rem(idx, ncol)))
      .filter(((i, j)) => is-accessible-roll(diagram, i, j))

    if accessible-rolls.len() == 0 {
      break
    }

    for (i, j) in accessible-rolls {
      diagram.at(i).at(j) = "."
    }
    total-rolls += accessible-rolls.len()
  }

  total-rolls
}
