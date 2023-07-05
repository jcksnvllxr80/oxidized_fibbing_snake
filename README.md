# rust package in python

how to write a rust package and use it in python, specifically Fibonacci.

## prereqs

- install python  
- install maturin
```shell
pip install maturin
```
- install rust  
 
## setup

```shell
maturin init  # choose pyo3
```
modify your new lib.rs to match the lib.rs in this repo

compile the new Rust package, and install it into virt env
```shell
maturin develop
maturin develop --release  # runs faster
```

modify python code to import the package and use it
now you're ready to run the python script

---

## Results

On my machine, the rust module runs 64 times faster (finding 30th fib number) than the same code written in python.
```text
Python μs per call: 160526.21 μs
Python ms per call: 160.53 ms

Rust μs per call: 2510.49 μs
Rust ms per call: 2.51 ms
```
