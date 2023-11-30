fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

fn run(input: &str) -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run(
            "",
        );
        assert_eq!(result, "todo!()")
    }
}
