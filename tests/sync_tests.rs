mod tests {
    use super::super::file_manager::*;

    #[test]
    fn test_file_io() {
        let path = "test.txt";
        let content = "Test content";

        save_file(path, content).expect("Save failed");
        let loaded = load_file(path).expect("Load failed");

        assert_eq!(content, loaded);
    }
}