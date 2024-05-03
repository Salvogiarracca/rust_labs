#[cfg(test)]
mod tests {
    use crate::slugify::{conv, slugify};

    #[test]
    fn convert_accented() {
        let test_ch = 'ä';
        let expected_ch = 'a';

        let actual_ch = conv(test_ch);

        assert_eq!(expected_ch, actual_ch)
    }

    #[test]
    fn convert_not_accented() {
        let test_ch = 'a';
        let expected_ch = 'a';

        let actual_ch = conv(test_ch);

        assert_eq!(expected_ch, actual_ch)
    }

    #[test]
    fn convert_not_alphabetic() {
        let test_ch = ')';
        let expected_ch = '-';

        let actual_ch = conv(test_ch);

        assert_eq!(expected_ch, actual_ch)
    }

    #[test]
    fn convert_not_known() {
        let test_ch = 'ῶ';
        let expected_ch = '-';

        let actual_ch = conv(test_ch);

        assert_eq!(expected_ch, actual_ch)
    }

    #[test]
    fn string_with_multiple_spaces() {
        let test_str = "Test test  test";
        let expected_str = "test-test-test";

        let actual_str = slugify(test_str);

        assert_eq!(expected_str, actual_str)
    }

    // TODO: define all other tests
    // - stringa con caratteri accentati
    // - stringa vuota
    // - stringa con più spazi consecutivi
    // - stringa con con più caratteri non validi consecutivi
    // - stringa con solo caratteri non validi
    // - stringa con spazio alla fine
    // - stringa con più caratteri non validi consecutivi alla fine
}