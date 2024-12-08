fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut is_safe = true;
        for i in 1..report.len() {
            if report[i] <= report[i - 1] || (report[i] - report[i - 1]) > 3 {
                is_safe = false;
                break;
            }
        }
    is_safe
}

fn safe_if_level_removed(report: &Vec<i32>, level: usize) -> bool {
    let mut new_report = report.clone();
    new_report.remove(level);
    is_report_safe(&new_report)
}

fn main() {
    let input = std::fs::read_to_string("src/input_big.txt").unwrap();
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect();
    let mut is_safe = true;
    let mut safe_reports = 0;
    reports.iter().for_each(|report| {
        let mut ascending = report.clone();
        if ascending.first() > ascending.last() {
            ascending.reverse()
        }
        print!("{:?} ", ascending);
        if is_report_safe(&ascending) {
            print!("safe\n");
            safe_reports += 1;
        } else {
            is_safe = false;
            let mut removed: usize = 0;
            for i in 1..ascending.len() - 1 {
                if safe_if_level_removed(&ascending, i) {
                    is_safe = true;
                    removed = i;
                    break;
                }
            }
            if is_safe {
                print!("safe removed {}\n", removed);
                safe_reports += 1;
            } else {
                print!("unsafe\n")
            }
        }
        
    });
    println!("Safe reports: {}", safe_reports);
}
