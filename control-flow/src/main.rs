fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    print!("The value of number is: {number}!");
    use_loop();
    loop_label();
}

fn use_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        print!("{counter}");
        if counter == 10 {
            break counter * 2;
        }
    };
    print!("\nThe result is {result}!")
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        print!("\ncount = {count}");
        let mut remaining = 10;

        loop {
            print!("\nremaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    print!("\nEnd count = {count}");
}
