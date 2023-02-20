# Rust README

## Structure

All files are located inside the rust/src folder. Inside it you can find a folder with the name of the topic covered by that part and, inside that folder, a mod.rs file with the code in it.

Come chapters are not located inside rust/src, and instead found in the root directory Rust. Those chapters are Chapter 12-An I/O project, Chapter 14.3-Cargo Workspaces.

Note that, specially when chapters get more difficult (specificaly from Chapter 13 until Chapter 19), all functions coded inside each lesson mod.rs file are preceded by the #[test] tag. Some of them are even called main when they are inside certain packages. This is done to allow easy running of certain functions. I know that this should not be coded this way, and also that test's usage is not to output data and see that output, it is checking results agains expected ones, but as stated before, this wrongdoing was my purpose when coding throughout The Book.

## Considerations

To create a new rust project, you must have Cargo package installed and call the command ``` cargo new _projectName_ ``` inside the folder where you want to have the project.

## References

All info is extracted from the [official Rust programming language docu](https://doc.rust-lang.org/book/title-page.html)
