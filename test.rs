// main.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading_and_parsing() {
        let expected = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let actual = read_file_and_parse("test_input.txt");
        
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].len(), actual[i].len());
            for j in 0..expected[i].len() {
                assert_eq!(expected[i][j], actual[i][j]);
            }
        }
    }
}

fn main() {
    // Main function, can be left empty for now
}
