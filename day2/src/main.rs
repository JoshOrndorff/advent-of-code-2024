fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("input file should exist");

    let reports: Vec<Vec<i128>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse().expect("num should parse"))
                .collect()
        })
        .collect();

    let num_safe_reports = reports.iter().filter(|&report| is_safe(report)).count();
    let num_damped_safe_reports = reports
        .iter()
        .filter(|&report| is_safe_with_dampener(report))
        .count();

    println!("{num_safe_reports}");
    println!("{num_damped_safe_reports}");
}

fn is_safe_with_dampener(report: &[i128]) -> bool {
    for i in 0..report.len() {
        let mut dampened_report = Vec::from(report);
        dampened_report.remove(i);

        if is_safe(&dampened_report) {
            return true;
        }
    }

    false
}

fn is_safe(report: &[i128]) -> bool {
    gradually_increasing(report) || gradually_decreasing(report)
}

fn gradually_increasing(report: &[i128]) -> bool {
    let mut prev = report[0];
    for level in &report[1..] {
        let difference = level - prev;
        if difference >= 1 && difference <= 3 {
            prev = *level;
        } else {
            return false;
        }
    }

    true
}

fn gradually_decreasing(report: &[i128]) -> bool {
    let mut prev = report[0];
    for level in &report[1..] {
        let difference = prev - level;
        if difference >= 1 && difference <= 3 {
            prev = *level;
        } else {
            return false;
        }
    }

    true
}
