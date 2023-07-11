use itertools::Itertools;

pub fn process_part_1(input: &str, req_uniq: usize) -> Option<usize> {
    let idx = input
        .as_bytes()
        .windows(req_uniq)
        .position(|chars| chars.iter().all_unique())
        .map(|idx| idx + req_uniq);
    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = process_part_1(&input, 4).unwrap();
        assert_eq!(result, 5);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = process_part_1(&input, 4).unwrap();
        assert_eq!(result, 6);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = process_part_1(&input, 4).unwrap();
        assert_eq!(result, 10);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = process_part_1(&input, 4).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_process_part_2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = process_part_1(&input, 14).unwrap();
        assert_eq!(result, 19);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = process_part_1(&input, 14).unwrap();
        assert_eq!(result, 23);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = process_part_1(&input, 14).unwrap();
        assert_eq!(result, 23);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = process_part_1(&input, 14).unwrap();
        assert_eq!(result, 29);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = process_part_1(&input, 14).unwrap();
        assert_eq!(result, 26);
    }
}
