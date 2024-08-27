#![feature(internal_output_capture)]

pub mod test_utils;

#[cfg(test)]
mod lox_test {
    use crate::test_utils::{default_filter, TraverseIterator};
    use rlox::lox;
    use std::{fs::File, io::Read, panic, sync::Arc};

    #[test]
    fn test_lox_scripts() {
        let iterator = TraverseIterator::new(
            "/Users/bytedance/Desktop/rlox/tests/scripts",
            &default_filter,
        );
        for path in iterator {
            let mut file = File::open(&path).unwrap();
            let mut script = String::new();
            file.read_to_string(&mut script).unwrap();

            let parts: Vec<&str> = script.split("------ output ------").collect();
            if parts.len() > 1 {
                std::io::set_output_capture(Some(Default::default()));

                let result = panic::catch_unwind(|| {
                    lox::run_code(parts[0]);
                });

                let captured = std::io::set_output_capture(None);

                if let Err(err) = result {
                    if let Some(captured) = err.downcast_ref::<&str>() {
                        assert!(false, "assertion failed: {}\n{}", path.display(), captured);
                        continue;
                    }
                    if let Some(captured) = err.downcast_ref::<String>() {
                        assert!(false, "assertion failed: {}\n{}", path.display(), captured);
                        continue;
                    }
                    assert!(
                        false,
                        "assertion failed: {}\n{}",
                        path.display(),
                        "Unknown error."
                    );
                    continue;
                }

                let captured = captured.unwrap();
                let captured = Arc::try_unwrap(captured).unwrap();
                let captured = captured.into_inner().unwrap();
                let captured = String::from_utf8(captured).unwrap();

                assert_eq!(captured.trim(), parts[1].trim(), "{}", path.display());
                continue;
            }

            let parts: Vec<&str> = script.split("------ error ------").collect();
            if parts.len() > 1 {
                std::io::set_output_capture(Some(Default::default()));

                let result = panic::catch_unwind(|| {
                    lox::run_code(parts[0]);
                });

                std::io::set_output_capture(None);

                if let Err(err) = result {
                    if let Some(captured) = err.downcast_ref::<&str>() {
                        assert_eq!(captured.trim(), parts[1].trim(), "{}", path.display());
                        continue;
                    }
                    if let Some(captured) = err.downcast_ref::<String>() {
                        assert_eq!(captured.trim(), parts[1].trim(), "{}", path.display());
                        continue;
                    }
                    assert!(
                        false,
                        "assertion failed: {}\n{}",
                        path.display(),
                        "Unknown error."
                    );
                    continue;
                }

                assert!(
                    false,
                    "assertion failed: {}\n{}",
                    path.display(),
                    "Expected error but nothing."
                );
                continue;
            }
        }
    }
}
