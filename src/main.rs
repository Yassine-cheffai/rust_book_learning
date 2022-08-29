fn main() {
    let x: String;
    {
        let name: String = "Yassine".to_string();
        x = name;
        println!("{x}");
    }
    println!("{x}");
}
