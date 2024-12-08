use regex::Regex;
use std::collections::BTreeMap;
use utils::read_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mem_section = read_memory("input.txt")?.join("");

    println!("[PART 1] {}", sum_muls1(&mem_section));

    println!("[PART 2] {}", sum_muls2(&mem_section));

    Ok(())
}

fn sum_muls1(s: &str) -> u32 {
    extract_muls(s).iter().map(|(_, [x, y])| x * y).sum()
}

fn sum_muls2(s: &str) -> u32 {
    let muls = extract_muls(s);

    let switches: BTreeMap<_, _> = s
        .match_indices("do()")
        .chain(s.match_indices("don't()"))
        .collect();

    muls.iter()
        .filter(|(loc, _)| {
            switches
                .range(..loc)
                .next_back()
                .map(|(_, &s)| s)
                .unwrap_or("do()")
                == "do()"
        })
        .map(|(_, [x, y])| x * y)
        .sum()
}

fn extract_muls(s: &str) -> Vec<(usize, [u32; 2])> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(s)
        .map(|c| (c.get(0).unwrap().start(), c.extract()))
        .map(|(loc, (_, [x, y]))| (loc, [x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()]))
        .collect()
}

fn read_memory(input_path: &str) -> std::io::Result<Vec<String>> {
    Ok(read_lines(input_path)?.map_while(Result::ok).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() -> Result<(), Box<dyn std::error::Error>> {
        let mem_section1 = read_memory("example1.txt")?.join("");
        assert_eq!(sum_muls1(&mem_section1), 161);

        let mem_section2 = read_memory("example2.txt")?.join("");
        assert_eq!(sum_muls2(&mem_section2), 48);

        Ok(())
    }
}
