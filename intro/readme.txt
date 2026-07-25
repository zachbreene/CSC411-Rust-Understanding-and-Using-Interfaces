Intro Assignment - Average Brightness / Fingerprint Groups

	C. Wyatt Polasek & Zach Breene

Acknowledgements:
Besides the code provided in the assignment document, some of the code we used we were 
able to modify from our previous lab (pnmdata). The only other resource we used during 
the implementation of the program was GitHub's Copilot, which was used to help fix code
in areas causing excessive trouble. Some questions were asked to Professor Daniels, and
some were asked in the TA Discord Server. 

Here are some websites we used for resources to teach us more about Rust as we learned to make these programs:
	https://doc.rust-lang.org/std/collections/struct.HashMap.html
	https://doc.rust-lang.org/std/io/struct.Stdin.html
	https://doc.rust-lang.org/std/io/trait.BufRead.html
	https://doc.rust-lang.org/std/vec/struct.Vec.html#method.join
	https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push
	https://doc.rust-lang.org/std/vec/struct.Vec.html#method.splitn
	https://stackoverflow.com/questions/36362020/what-is-unwrap-in-rust-and-what-is-it-used-for
	https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default
	https://doc.rust-lang.org/std/string/trait.ToString.html
	https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace
	https://doc.rust-lang.org/stable/nightly-rustc/rustc_lexer/fn.is_whitespace.html
	https://www.educative.io/answers/what-is-the-trim-function-in-rust
	https://docs.rs/csc411_image/latest/csc411_image/
These websites are also listed as resources in the comments in our code.

For this assignment, we have correctly implemented both Part A and Part B. Part A of
the assignment was a program that could take in a grayscale image and return its average 
brightness. Part B of the assignment was a program that would take input of names and
associated fingerprints, and would return "fingerprint groups" for each fingerprint with 
more than one associated name. On account of the Gradescope tests, all tests were passed,
including the following: White image, Brightness compilation, Large input,
Binary fingerprints, and Fgroups compilation.


Here are some possible problems that could be solved using fgroups:

  - Fgroups could be used to analyze genetic sequences to find correlations between 
	different people with the same conditions.

  - Fingerprints could be made from students' assignments to check for plagiarism.

  - Fgroups could be used to track down bugs in code by fingerprinting error reports 
	and grouping them based on location.

  - Schools could use fgroups to group students by extracurriculars to help create 
	mailing groups to send important information or reminders only to students in
	the associated extracurricular group.

It took us approximately 12-14 hours to design and implement the assignment.
