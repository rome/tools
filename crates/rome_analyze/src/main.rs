fn main() {
    // let mut vec = vec!["img1.png", "img2.png", "img10.png", "img12.png"];
    // vec.sort();
    // println!("{:?}", vec);
    let mut a = "testtest";
    let mut b = a.chars();

    while let Some(ch) = b.next() {
        println!("{:?}", b.size_hint());
    }
    // let mut cc = b.clone();
    // b.next();
    // b.next();
    // println!("{:?}", b.peek());
    // println!("{:?}", cc.peek());
}
