use std::path::Path;

fn main() {
    let paths = [
        r"\\?\",
        r"\\.\",
        r"\\?",
        r"\\.",
        r"C:",
        r"C:/",
        r"C:\",
    ];

    for left in paths {
        for right in paths {
            let p = Path::new(left);
            let q = Path::new(right);
            let r = p.join(q);
            let joined = r.to_str().expect("valid Unicode");
            println!("{left:4}  +  {right:4}  =  {joined}");
        }
    }
}
