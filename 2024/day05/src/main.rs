use std::collections::HashMap;
use utils::read_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ordering_rules, updates) = read_instructions("input.txt")?;

    let (valid_updates, mut invalid_updates): (Vec<_>, Vec<_>) = updates
        .into_iter()
        .partition(|update| update.is_valid(&ordering_rules));

    println!("[PART 1] {}", sum_middle_page_numbers(&valid_updates));

    invalid_updates
        .iter_mut()
        .for_each(|update| update.reorder(&ordering_rules));

    println!("[PART 2] {}", sum_middle_page_numbers(&invalid_updates));

    Ok(())
}

type PageOrderingRule = (u32, u32);

struct Update {
    pages: Vec<u32>,
    page_index: HashMap<u32, usize>,
}

impl Update {
    fn is_valid(&self, constraints: &[PageOrderingRule]) -> bool {
        constraints.iter().all(|(a, b)| {
            if let (Some(a_idx), Some(b_idx)) = (self.page_index.get(a), self.page_index.get(b)) {
                a_idx < b_idx
            } else {
                true
            }
        })
    }

    fn reorder(&mut self, constraints: &[PageOrderingRule]) {
        let mut is_valid = false;

        while !is_valid {
            is_valid = true;

            for &(a, b) in constraints {
                if let (Some(&a_idx), Some(&b_idx)) =
                    (self.page_index.get(&a), self.page_index.get(&b))
                {
                    if a_idx > b_idx {
                        is_valid = false;

                        self.pages.swap(a_idx, b_idx);

                        self.page_index.insert(a, b_idx);
                        self.page_index.insert(b, a_idx);
                    }
                }
            }
        }
    }
}

fn sum_middle_page_numbers(updates: &[Update]) -> u32 {
    updates
        .iter()
        .map(|update| update.pages[update.pages.len() / 2])
        .sum()
}

fn read_instructions(input_path: &str) -> std::io::Result<(Vec<PageOrderingRule>, Vec<Update>)> {
    let mut line_iter = read_lines(input_path)?.map_while(Result::ok);

    let ordering_rules = line_iter
        .by_ref()
        .take_while(|s| !s.is_empty())
        .filter_map(|s| {
            s.split_once('|')
                .and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))
        })
        .collect();

    let updates = line_iter
        .map(|s| {
            let pages: Vec<_> = s.split(',').filter_map(|x| x.parse().ok()).collect();
            let page_index = pages
                .iter()
                .enumerate()
                .map(|(i, &page)| (page, i))
                .collect();
            Update { pages, page_index }
        })
        .collect();

    Ok((ordering_rules, updates))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let (ordering_rules, updates) = read_instructions("example.txt")?;

        let (valid_updates, mut invalid_updates): (Vec<_>, Vec<_>) = updates
            .into_iter()
            .partition(|update| update.is_valid(&ordering_rules));

        assert_eq!(sum_middle_page_numbers(&valid_updates), 143);

        invalid_updates
            .iter_mut()
            .for_each(|update| update.reorder(&ordering_rules));

        assert_eq!(sum_middle_page_numbers(&invalid_updates), 123);
        Ok(())
    }
}
