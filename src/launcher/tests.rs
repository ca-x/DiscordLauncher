mod test {
    use super::super::find_executable_in_path;
    use super::super::launch_discord;

    #[test]
    fn test_find_executable_in_path() {
        let result = find_executable_in_path("goland");
        assert!(result.is_some());
        println!("{:?}", result);
    }

    #[test]
    fn test_launch_discord() {
        launch_discord();
    }
}