fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("Not Found any files.");
    
    println!("File name: {args}");
}
