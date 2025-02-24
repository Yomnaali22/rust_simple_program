# 📌 Rust String Concatenation: Ownership & Borrowing

## 📖 Task Details
In this task, students will create a simple Rust program that demonstrates the concepts of **ownership, borrowing, and references**. The program will take two strings as input, concatenate them, and then print the result **without violating any ownership rules**.

---

## 🚀 Program Code
```rust
fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new(); // mutable variable
    result.push_str(str1);
    result.push_str(str2);
    return result;
}

fn main() {
    let string1: String = String::from("hello");
    let string2: String = String::from(" world");
    let concatenated_string = concatenate_strings(&string1, &string2); // & for borrowing 
    println!("{}", concatenated_string);
}
```

---

## 📝 Explanation
- The function **`concatenate_strings`** takes two **borrowed** string slices (`&str`).
- This prevents **ownership transfer**, allowing `string1` and `string2` to still be used after the function call.
- A **new `String`** is created inside the function, modified with `push_str()`, and returned.
- In `main()`, the function is called using **borrowed references** (`&string1` and `&string2`).
- This ensures **no ownership violations** while efficiently concatenating the strings.

---

## ✅ Expected Output
```
hello world
```

---

## 📌 Key Concepts Demonstrated
- **Ownership**: The function does not take ownership of `string1` and `string2`, allowing them to be reused.
- **Borrowing & References**: The function parameters use `&str`, enabling safe and efficient string handling.
- **String Manipulation**: Using `push_str()` to append borrowed string slices to a mutable `String`.

---

## 🔥 Running the Program
### 1️⃣ Ensure you have Rust installed:
```sh
rustc --version
```
If Rust is not installed, install it using:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2️⃣ Compile and Run the Program:
```sh
rustc main.rs
./main
```
OR using Cargo:
```sh
cargo run
```

---