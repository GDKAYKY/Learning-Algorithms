pub fn main() {
    let x: i32 = read_input("Enter first number: ");
    let y: i32 = read_input("Enter second number: ");
    let result = karatsuba_multiplication(x, y);
    println!("The product of {} and {} is {}", x, y, result);
}

pub fn read_input(prompt: &str) -> i32 {
    print!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

pub fn count_digits(mut n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    if n < 0 {
        n = -n;
    }

    let mut count = 0;

    while n > 0 {
        n /= 10;
        count += 1;
    }
    return count;
}

pub fn karatsuba_multiplication(x: i32, y: i32) -> i32 {
    let n = count_digits(x).max(count_digits(y));
    // Split X and Y
    let a = x / n; // Higher half of x
    let b = x % n; // Lower half of x

    let c = y / n; // Higher half of y
    let d = y % n; // Lower half of y

    // Step 1: Compute a.c
    let ac = a * c;
    // Step 2: Compute b.d
    let bd = b * d;
    // Step 3: Compute (a + b)(c + d)
    let abcd = (a + b) * (c + d);
    // Step 4: Compute (a + b)(c + d) - ac - bd
    let result = abcd - ac - bd;

    return result;
}
