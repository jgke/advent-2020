#[allow(dead_code)]
pub fn as_groups(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut grouped = Vec::new();
    let mut groups = Vec::new();
    for line in lines {
        if line.is_empty() && grouped.len() > 0 {
            groups.push(grouped);
            grouped = Vec::new();
        } else {
            grouped.push(line);
        }
    }

    if grouped.len() > 0 {
        groups.push(grouped);
    }

    groups
}
