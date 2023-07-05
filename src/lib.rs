use pyo3::prelude::*;

/// Calculate the nth Fibonacci number.
#[pyfunction]
fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}


/// Fast Fibonacci number calculation.
#[pymodule]
fn oxidized_fibbing_snake(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib, m)?)?;
    Ok(())
}