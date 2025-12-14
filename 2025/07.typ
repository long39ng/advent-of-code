= Day 7: Laboratories

A tachyon beam enters the manifold at the location marked S; tachyon beams always move *downward*. Tachyon beams pass freely through empty space (`.`). However, if a tachyon beam encounters a splitter (`^`), the beam is stopped; instead, a new tachyon beam continues from the immediate left and from the immediate right of the splitter.

How many times will the beam be split?

#let diagram = read("07-input.txt").trim().split().map(s => s.clusters())

#let start = diagram.at(0).position(c => c == "S")

#let splitter-rows = (
  diagram
    .slice(1)
    .map(row => row.enumerate().filter(((_, c)) => c == "^").map(((j, _)) => j))
    .filter(row => row.len() != 0)
)

Answer:
#{
  splitter-rows
    // beam === j
    .fold(
      ((start,), 0),
      ((beams, n-splits), splitters) => {
        n-splits += beams.filter(j => j in splitters).len()

        let new-beams = beams
          .map(j => {
            if j in splitters {
              ((j - 1), (j + 1))
            } else {
              (j,)
            }
          })
          .join()
          .dedup()

        (new-beams, n-splits)
      },
    )
    .at(1)
}

== Part Two

With a quantum tachyon manifold, only a *single tachyon particle* is sent through the manifold. A tachyon particle takes *both* the left and right path of each splitter encountered.

Since this is impossible, the manual recommends the many-worlds interpretation of quantum tachyon splitting: each time a particle reaches a splitter, it's actually *time itself* which splits. In one timeline, the particle went left, and in the other timeline, the particle went right.

To fix the manifold, what you really need to know is the *number of timelines* active after a single particle completes all of its possible journeys through the manifold.

In total, how many different timelines would a single tachyon particle end up on?

Answer:
#{
  // n timelines = sum(n beams at splits)
  // * not dedup by position
  splitter-rows
    // beam === (j, n)
    .fold(
      ((start, 1),),
      (beams, splitters) => {
        beams
          .map(((j, n)) => {
            if j in splitters {
              ((j - 1, n), (j + 1, n))
            } else {
              ((j, n),)
            }
          })
          .join()
          // Sum n by j
          .fold(
            ((0, 0),),
            (counts, (j, n)) => {
              let count-j-idx = counts.position(((key, _)) => key == j)
              if count-j-idx == none {
                counts.push((j, n))
              } else {
                counts.at(count-j-idx).at(1) += n
              }
              counts
            },
          )
      },
    )
    .map(((_, n)) => n)
    .sum()
}
