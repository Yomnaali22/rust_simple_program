fn main() {
    println!("Hello, world!");
}


/*
Task Details

In this task, students will create a simple Rust program that demonstrates the concepts of ownership, borrowing, and references. The program will take two strings as input, concatenate them, and then print the result without violating any ownership rules.

Steps

Create a function called concatenate_strings that takes two string slices as arguments and returns a new String as the result of concatenating the two input strings.
Inside the concatenate_strings function, create a new String called result. Use the push_str() method to append the contents of the first input string slice, followed by the second input string slice.
Return the result string from the function.
In the main function, create two String variables, string1 and string2, and initialize them with appropriate values.
Call the concatenate_strings function with references to string1 and string2 as arguments (using string slices). Store the result in a new variable called concatenated_string.
Print the concatenated_string variable to the console.
Compile and run the program to ensure it works as expected.
Checklist

Write the concatenate_strings function signature.
Implement the concatenate_strings function.
Initialize two String variables in the main function.
Call the concatenate_strings function with string slices of the variables.
Print the result to the console.
Compile and run the program to test its functionality.

*/