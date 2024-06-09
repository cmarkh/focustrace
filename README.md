# focustrace
Small utility to print a backtrace containing only functions called from the current crate on panic. 

It's a simple extension of the backtrace crate. 

## Usage 
```
fn main() {
  focustrace::setup();
}
```

## Example output on panic
```
panicked at src/lib.rs:43:9:
Test panic

src/lib.rs:4: focustrace::setup::{{closure}}::haa6742a590bf138b
src/lib.rs:43: focustrace::tests::test_setup::h6f4a089fcd9ad7b4
src/lib.rs:41: focustrace::tests::test_setup::{{closure}}::ha937ffcfc3bdf67a
```
