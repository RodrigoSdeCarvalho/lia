#[macro_export]
macro_rules! benchmark {
    ($($token:tt)+) => {
        {
            let _instant = std::time::Instant::now();
            let _result = {
                $($token)+
            };

            let benchmark_log = format!("[Benchmark] {} took {}ms", stringify!($($token)+), _instant.elapsed().as_millis());
            println!("{}", benchmark_log);

            _result
        }
    }
}

#[cfg(test)]
mod tests {
    use benchmark_macro::benchmark;

    fn stress_test() -> String {
        for _ in 0..1000000 {
            let _ = 1 + 1;
        }

        String::from("I'm stressed!")
    }

    fn parameterized_stress_test(n: u32) -> String {
        for _ in 0..n {
            let _ = 1 + 1;
        }

        String::from("I'm stressed!")
    }

    fn multi_parameterized_stress_test(n: u32, m: u32) -> String {
        for _ in 0..n {
            for _ in 0..m {
                let _ = 1 + 1;
            }
        }

        String::from("I'm stressed!")
    }

    #[benchmark]
    fn macro_stress_test() -> String {
        stress_test()
    }

    #[test]
    fn test_benchmark() {
        let result = benchmark! {
            stress_test()
        };
        assert_eq!(result, String::from("I'm stressed!"));

        let result = benchmark! {
            parameterized_stress_test(1000000)
        };
        assert_eq!(result, String::from("I'm stressed!"));

        let result = benchmark! {
            multi_parameterized_stress_test(1000, 1000)
        };
        assert_eq!(result, String::from("I'm stressed!"));

        let result = macro_stress_test();
        assert_eq!(result, String::from("I'm stressed!"));
    }
}
