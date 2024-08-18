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
            println!("{:?} + {:?} = {:?}", p, q, p.join(q));
        }
    }
}
