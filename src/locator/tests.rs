mod test{
    use super::super::find_executable_in_path;
    #[test]
    fn test_find_executable_in_path(){
        let result = find_executable_in_path("goland");
        assert!(result.is_some());
        println!("{:?}", result);
    }
}