use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn summer(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right && right > 0 {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((nums[left], nums[right]));
        } else if sum > target {
            right -= 1;
        } else if sum < target {
            left += 1;
        }
    }
    None
}

pub fn one() -> Result<(), std::io::Error> {
    let file = File::open("1_input")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<i32> = reader
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
    lines.sort();
    let (left, right) = summer(&lines, 2020).unwrap();
    println!(
        "{} + {} = 2020, {} * {} = {}",
        left,
        right,
        left,
        right,
        left * right
    );
    for line in &lines {
        if let Some((left, right)) = summer(&lines, 2020 - line) {
            println!(
                "{} + {} + {} = 2020, {} * {} * {} = {}",
                left,
                right,
                line,
                left,
                right,
                line,
                left * right * line
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_1::summer;
    #[test]
    fn it_works() {
        let mut lines = vec![1721, 979, 366, 299, 675, 1456];
        lines.sort();
        assert_eq!(Some((299, 1721)), summer(lines));
    }
}
