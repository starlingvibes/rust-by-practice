fn main() {
    let x: i32 = 5;

    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x: i32 = 42;
    println!("The value of x is {}", x);
}