Square Root Difference Calculator
Overview

This project, Square Root Difference Calculator, is a Rust-based program that calculates and compares the differences between the standard square root (sqrt_number()) and a custom square root approximation (sqrt_funktion()). It iterates over a large range of numbers (up to 1 million), computes the difference between the two square roots for each number, and writes the results to a file (result.txt).
Features

    Efficient Square Root Calculation: Uses both the standard Rust sqrt() function and a custom approximation method.
    File Output: Writes the number, its square roots, and the percentage difference between them to an output file.
    Average Difference Calculation: Computes the average difference between the two square root methods over all iterations.
    Optimized Performance: Reduced redundant calculations and minimized file I/O overhead by optimizing file handling and caching computations.

Usage

Simply run the main() function, and the program will:

    Iterate through numbers from 1 to 1 million.
    Calculate both the real and approximated square root of each number.
    Write the results (number, square roots, and their difference) to result.txt.
    Write the average percentage difference between the two methods at the end of the file.
    If the file already exists, it clears the file and writes to it again.
    
Prerequisites

    Rust: Ensure you have Rust installed to compile and run the program.
    
    After execution, check result.txt for the output.
