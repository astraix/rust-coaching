fn main() {
    let file_size = 3120; // bytes
    let file_size = file_size / 1024; //KiB (shadowing)
    println!("File size is {file_size} KiB");
}
