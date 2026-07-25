<h1 align=center> Intro to Rust: Brightness & Fgroups </h1>
<h2 align=center> A CSC 411: Computer Organization Assignment by Zach Breene & C. Wyatt Polasek </h2>
<h4 align=center> Created at the University of Rhode Island, September 2023 </h4>

## Introduction
The purpose of this assignment was to transition into programming with the Rust language, practice identifying helpful interfaces, and handle multiple representations of numbers. The project is divided into two separate applications: `brightness`, which calculates the average brightness of a grayscale image, and `fgroups`, which processes and groups matching string fingerprints.

---

## Implementation + Functions
### intro/brightness/src/main.rs

This project directory contains the logic for the grayscale image processor. <br>

&emsp; ***Image Processing Method***

* The program utilizes the `csc411_image` crate to read portable graymap (PGM) files.
* It accepts the image either through a file name provided as a single command-line argument or directly from standard input.
* The script calculates the denominator and iterates over the pixels to determine the average brightness, printing the result to standard output using exactly three digits after the decimal point.
* Badly formed inputs, non-graymap files, or receiving more than one argument results in a halt and a standard error message printed to stderr.

### intro/fgroups/src/main.rs

This project directory contains the logic for identifying and grouping shared fingerprints. <br>

&emsp; ***Data Structure & Invariant Method***

* The core data structure relies on the Rust standard library to build a `HashMap<String, Vec<String>>`.
* In this structure, the concrete types dictate that the keys represent the string fingerprints, and the values are vectors containing the associated string names.
* As the input lines are processed partway, the invariant holds that the HashMap contains unique fingerprints as keys, and the Vectors are associated with each unique fingerprint to contain names that share it.

&emsp; ***Group Computation Method***

* After all input lines are read from standard input, the script iterates through the HashMap.
* It filters out and ignores any fingerprints that have only one associated name.
* For fingerprints associated with two or more names, it prints the names separated by newlines, implying fingerprint groups are formed containing names with their associated fingerprint.

---

## Part C: Problem Solving with fgroups
A working implementation of `fgroups` can be utilized to solve real-world system problems, specifically by identifying duplicate files on a computer. By using a command-line utility like `find` alongside its `exec` option, you can generate a cryptographic hash for a large batch of files. If you pass these hashes (as fingerprints) and the file paths (as names) into standard input, `fgroups` will automatically cluster and output the paths of files that share the exact same content, allowing users to easily delete redundancies.

---

## How To Run
**IMPORTANT: Ensure you have a working Rust environment and the jpeg toolkit from https://docs.rs/csc411_image/latest/csc411_image/ installed for testing images.**

To run these implementations, navigate to either the `intro/brightness` or `intro/fgroups` directory. 
* **Brightness:** Run the compiled binary using a file argument (`target/release/brightness cellar.pgm`) or pipe a file via standard input (`djpeg -grayscale bear.jpg | target/release/brightness`).
* **Fgroups:** Run the compiled binary and pass your formatted text via standard input, ensuring each line contains a fingerprint sequence (up to 512 characters) followed by whitespace and a name.

---

## Contribution
This project was developed utilizing pair programming. 
* **Partners:** Zach Breene and C. Wyatt Polasek.
* **Implementation Status:** Both the `brightness` and `fgroups` programs have been implemented to correctly handle valid inputs, ignore or truncate bad data without crashing, and format outputs accurately.
