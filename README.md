# Three ways of mocking modules in Rust

## What should be mocked?

In this example, the function Utc::now should be mocked, so we can fix it to a certain timestamp.

## Mock with an enum

The first approach was to pull out the functionality which should be mocked into an enum Trait. This trait can then be implemented with a real and mocked implementation. Which implementation should be used can be passed to the function using the turbofish notation.

[Example using the Enum](https://github.com/veronika-hillebrand-mw/rust-testing-mock-modules/blob/main/src/time_util_mock_enum.rs)

## Mock with a struct

In our example, it would be nice to be able to alter the value returned by `Utc::now` from one test to another. We can use a struct with a timestamp field for that. Now we have to pass the implementation as an argument.

[Example using the Struct](https://github.com/veronika-hillebrand-mw/rust-testing-mock-modules/blob/main/src/time_util_mock_struct.rs)

## Mock with mockall library

Last not least, there is a library which takes over some of the boilerplate and delivers syntactic sugar: [Mockall](https://docs.rs/mockall/latest/mockall/index.html)

[Example using Mockall](https://github.com/veronika-hillebrand-mw/rust-testing-mock-modules/blob/main/src/time_util_mockall.rs)