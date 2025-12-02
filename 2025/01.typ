= Day 1: Secret Entrance

A rotation starts with an `L` or `R` which indicates whether the rotation should be to the *left* (toward lower numbers) or to the *right* (toward higher numbers). Then, the rotation has a *distance* value which indicates how many clicks the dial should be rotated in that direction.

Because the dial is a circle, turning the dial *left from `0`* one click makes it point at `99`. Similarly, turning the dial *right from `99`* one click makes it point at `0`.

The dial starts by pointing at 50.

The actual password is the *number of times the dial is left pointing at 0 after any rotation* in the sequence.

What's the actual password to open the door?

#let rotations = (
  read("01-input.txt")
    .trim()
    .split("\n")
    .map(s => {
      let (direction, distance) = (s.slice(0, 1), int(s.slice(1)))
      if direction == "L" { -distance } else { distance }
    })
)

Answer:
#{
  rotations
    .fold((50, 0), ((position, n_zeroes), rotate_by) => {
      let new_position = calc.rem-euclid(position + rotate_by, 100)
      (
        new_position,
        n_zeroes + int(new_position == 0),
      )
    })
    .at(1)
}

== Part Two

"method 0x434C49434B" means you're actually supposed to count the number of times *any click* causes the dial to point at 0, regardless of whether it happens during a rotation or at the end of one.

Using password method 0x434C49434B, what is the password to open the door?

Answer:
#{
  rotations
    .fold((50, 0), ((position, n_zeroes), rotate_by) => {
      let new_position = calc.rem-euclid(position + rotate_by, 100)
      (
        new_position,
        n_zeroes
          + calc.abs(calc.div-euclid(position + rotate_by, 100))
          // Correct overcounting by 1 when current position is 0 and rotating CCW
          - int(position == 0 and rotate_by < 0)
          // Correct undercounting by 1 when rotating CCW and new position is 0
          + int(rotate_by < 0 and new_position == 0),
      )
    })
    .at(1)
}
