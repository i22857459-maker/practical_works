// 3
fn solve3() {
    let x: i32 = 10;
    let y: i32 = 5;

    {
        let y: i32 = 5;
        println!(
            "Inner scope value of x is {} and value of y is {}",
            x, y
        );
    }

    println!(
        "Outer scope value of x is {} and value of y is {}",
        x, y
    );
}

// 4
fn define_x() -> &'static str {
    "hello"
}

fn solve4() {
    let x = define_x();
    println!("{}, world", x);
}

// 6
fn solve6() {
    let x = 1;
    let x = x + 6;
    let x = x + 3;

    let _y = 4;
    let y = "I can also be bound to text!";

    println!("x is {}", x);
    println!("y is {}", y);
    println!("Success!");
}

fn main() {
    solve3();
    solve4();
    solve6();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_x() {
        assert_eq!(define_x(), "hello");
    }

    #[test]
    fn test_solve3() {
        solve3();
    }

    #[test]
    fn test_solve4() {
        solve4();
    }

    #[test]
    fn test_solve6() {
        solve6();
    }
}
