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