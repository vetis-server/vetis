mod rt_tests {
    use smol_macros::test;

    use crate::rt::SmolExecutor;

    #[test]
    fn test_smol_executor_new() {
        let executor = SmolExecutor::new();
        let _ = executor;
    }

    #[test]
    fn test_smol_executor_default() {
        let executor = SmolExecutor::default();
        let _ = executor;
    }

    #[test]
    fn test_smol_executor_clone() {
        let executor = SmolExecutor::new();
        let cloned = executor.clone();
        let _ = (executor, cloned);
    }

    #[test]
    fn test_smol_executor_debug() {
        let executor = SmolExecutor::new();
        let _ = format!("{:?}", executor);
    }
}
