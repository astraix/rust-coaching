fn main() {
    let max_file_size_allowed = 20000;
    let maintenance_mode = false;

    let file_size = 0;
    let upload_allowed = file_size > 0 && file_size <= max_file_size_allowed && !maintenance_mode;
    println!("Upload allowed: {upload_allowed}");

    let file_size = 12304;
    let upload_allowed = file_size > 0 && file_size <= max_file_size_allowed && !maintenance_mode;
    println!("Upload allowed: {upload_allowed}");

    let file_size = max_file_size_allowed;
    let upload_allowed = file_size > 0 && file_size <= max_file_size_allowed && !maintenance_mode;
    println!("Upload allowed: {upload_allowed}");

    let file_size = 100;
    let maintenance_mode = true;
    let upload_allowed = file_size > 0 && file_size <= max_file_size_allowed && !maintenance_mode;
    println!("Upload allowed: {upload_allowed}");

    let file_size = 10011111;
    let maintenance_mode = false;
    let upload_allowed = file_size > 0 && file_size <= max_file_size_allowed && !maintenance_mode;
    println!("Upload allowed: {upload_allowed}");
}
