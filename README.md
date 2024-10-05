Square Root Difference Calculator
Overview

This project, Square Root Difference Calculator, is a Rust-based program that calculates and compares the differences between the standard square root (sqrt_number()) and a custom square root approximation (sqrt_funktion()). It iterates over a range of numbers (up to 1 million), computes the difference between the two square roots for each number, and writes the results to a file (result.txt).
Features

    Efficient Square Root Calculation: Uses both the standard Rust sqrt() function and a custom approximation method.
    File Output: Writes the number, its square roots, and the percentage difference between them to an output file.
    Average Difference Calculation: Computes the average difference between the two square root methods over all iterations.
    Optimized Performance: Reduces redundant calculations and minimizes file I/O overhead by optimizing file handling and caching computations.

Usage

To run the program:

    The program will iterate through numbers from 1 to 1 million.
    It will calculate both the real and approximated square root of each number.
    Results (the number, square roots, and their difference) will be written to result.txt.
    The average percentage difference between the two methods will be written at the end of the file.

Prerequisites

    Rust: Ensure you have Rust installed to compile and run the program.

How to Run

To compile and run the program:

cargo run

After execution, check the result.txt file for the output.
