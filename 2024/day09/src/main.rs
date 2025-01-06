use std::fs::read_to_string;

struct DiskRegion {
    pos: usize,
    len: usize,
}

impl DiskRegion {
    fn first_block_pos(&self) -> usize {
        self.pos
    }
    fn last_block_pos(&self) -> usize {
        self.pos + self.len - 1
    }
    fn sum_block_pos(&self) -> usize {
        (self.first_block_pos() + self.last_block_pos()) * self.len / 2
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut disk_regions = read_disk_map("input.txt")?;

    compact(&mut disk_regions);
    println!("[PART 2] {}", calculate_checksum(&disk_regions));

    Ok(())
}

fn compact(disk_regions: &mut [DiskRegion]) {
    for used_idx in (0..disk_regions.len()).rev().step_by(2) {
        for free_idx in (1..disk_regions.len()).step_by(2) {
            if disk_regions[free_idx].pos < disk_regions[used_idx].pos
                && disk_regions[free_idx].len >= disk_regions[used_idx].len
            {
                disk_regions[used_idx].pos = disk_regions[free_idx].pos;
                disk_regions[free_idx].pos += disk_regions[used_idx].len;
                disk_regions[free_idx].len -= disk_regions[used_idx].len;
                break;
            }
        }
    }
}

fn calculate_checksum(disk_regions: &[DiskRegion]) -> usize {
    disk_regions
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(id, disk_region)| id / 2 * disk_region.sum_block_pos())
        .sum()
}

fn read_disk_map(input_path: &str) -> std::io::Result<Vec<DiskRegion>> {
    Ok(read_to_string(input_path)?
        .chars()
        .filter_map(|c| c.to_digit(10).map(|n| n as usize))
        .scan(0, |pos, len| {
            let disk_block = DiskRegion { pos: *pos, len };
            *pos += len;
            Some(disk_block)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let mut disk_regions = read_disk_map("example.txt")?;

        compact(&mut disk_regions);
        assert_eq!(calculate_checksum(&disk_regions), 2858);

        Ok(())
    }
}
