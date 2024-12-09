use ndarray::{array, s, Array2};
use utils::read_lines;

fn main() -> anyhow::Result<()> {
    let a = read_2d_array("input.txt")?;

    println!("[PART 1] {}", focal_count_xmas(&a));

    println!("[PART 2] {}", focal_count_x_mas(&a));

    Ok(())
}

fn focal_count_xmas(a: &Array2<char>) -> usize {
    let xmas = array!['X', 'M', 'A', 'S'];
    let xmas_rev = array!['S', 'A', 'M', 'X'];

    let n_horizontal = a
        .windows((1, 4))
        .into_iter()
        .filter(|w| w.row(0) == xmas || w.row(0) == xmas_rev)
        .count();

    let n_vertical = a
        .windows((4, 1))
        .into_iter()
        .filter(|w| w.column(0) == xmas || w.column(0) == xmas_rev)
        .count();

    let n_diagonal: usize = a
        .windows((4, 4))
        .into_iter()
        .map(|w| {
            let w_flipped = w.slice(s![.., ..; -1]);
            let diags = [w.diag(), w_flipped.diag()];
            diags.iter().filter(|&d| d == xmas || d == xmas_rev).count()
        })
        .sum();

    n_horizontal + n_vertical + n_diagonal
}

fn focal_count_x_mas(a: &Array2<char>) -> usize {
    let mas = array!['M', 'A', 'S'];
    let mas_rev = array!['S', 'A', 'M'];

    a.windows((3, 3))
        .into_iter()
        .filter(|w| {
            let w_flipped = w.slice(s![.., ..; -1]);
            let diags = [w.diag(), w_flipped.diag()];
            diags.iter().all(|&d| d == mas || d == mas_rev)
        })
        .count()
}

// https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#conversions-from-nested-vecsarrays
fn read_2d_array(input_path: &str) -> anyhow::Result<Array2<char>> {
    let mut data = Vec::new();

    let mut line_iter = read_lines(input_path)?
        .map_while(Result::ok)
        .map(|s| s.chars().collect::<Vec<_>>());

    data.extend_from_slice(&line_iter.next().unwrap());
    let ncol = data.len();
    let mut nrow = 1;

    line_iter.for_each(|cs| {
        data.extend_from_slice(&cs);
        nrow += 1;
    });

    Ok(Array2::from_shape_vec((nrow, ncol), data)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() -> anyhow::Result<()> {
        let a = read_2d_array("example.txt")?;
        assert_eq!(focal_count_xmas(&a), 18);
        assert_eq!(focal_count_x_mas(&a), 9);
        Ok(())
    }
}
