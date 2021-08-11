use behavior;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        behavior::add_one(num)
    );
}
