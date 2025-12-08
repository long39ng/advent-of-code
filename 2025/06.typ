= Day 6: Trash Compactor

The math worksheet (your puzzle input) consists of a list of *problems*; each problem has a group of numbers that need to be either *added* (`+`) or *multiplied* (`*`) together.

What is the grand total found by adding together all of the answers to the individual problems?

#let (..numbers, operations) = read("06-input.txt").trim().split("\n")
#let operations = operations.split()

Answer:
#{
  let rows = numbers.map(s => s.split())
  rows
    .at(0)
    .zip(..rows.slice(1))
    .zip(operations)
    .map(((nums, op)) => {
      if op == "+" { nums.map(int).sum() } else { nums.map(int).product() }
    })
    .sum()
}

== Part Two

Cephalopod math is written *right-to-left in columns*. Each number is given in its own column, with the most significant digit at the top and the least significant digit at the bottom.

What is the grand total found by adding together all of the answers to the individual problems?

Answer:
#{
  let cols = numbers.map(s => s.rev().clusters())
  cols
    .at(0)
    .zip(..cols.slice(1))
    .map(chars => chars.join().trim())
    .split("")
    .zip(operations.rev())
    .map(((nums, op)) => {
      if op == "+" { nums.map(int).sum() } else { nums.map(int).product() }
    })
    .sum()
}
