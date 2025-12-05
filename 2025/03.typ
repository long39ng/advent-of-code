= Day 3: Lobby

The batteries are each labeled with their joltage rating, a value from `1` to `9`.

Within each bank, you need to turn on *exactly two* batteries; the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on.

Find the maximum joltage possible from each bank; what is the total output joltage?

#let battery-banks = read("03-input.txt").trim().split("\n").map(s => s.clusters().map(int))

Answer:
#let which-max-first(xs) = {
  xs.enumerate().sorted(key: ((i, x)) => (-x, i)).first()
}
#let max-joltage(n-digits) = {
  battery-banks
    .map(batteries => {
      let joltage = 0
      let tail-len = n-digits - 1

      while tail-len >= 0 {
        let head = which-max-first(batteries.slice(0, batteries.len() - tail-len))

        joltage += head.at(1) * calc.pow(10, tail-len)

        batteries = batteries.slice(head.at(0) + 1)
        tail-len -= 1
      }

      joltage
    })
    .sum()
}
#{
  max-joltage(2)
}

== Part Two

Now, you need to make the largest joltage by turning on *exactly twelve* batteries within each bank.

What is the new total output joltage?

Answer:
#{
  max-joltage(12)
}
