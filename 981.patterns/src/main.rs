// Using patterns in conjunction with match() expressions and other 
// constructs gives you more control over a program's control flow

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    println!("\n Destructuring +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");

    let point = (3, 5);
    print_coordinates(&point);

    println!("\n structs +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");

    let p = Point { x: 1, y: 7 };

    let Point { x, y } = p;
    assert_eq!(1, x);
    assert_eq!(7, y);

    // I like this method the best
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }

    // Destructuring Enums - using patterns to break down the variants
    //
//    let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }

        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }

        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println! (
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // @ Bindings
    let msgb = MessageBind::Hello { id: 8 };

    match msgb { 
        MessageBind::Hello {
            id: id_variable @ 3..=7, // 
        } => println!("Found an id in range: {}", id_variable),
        MessageBind::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageBind::Hello { id } => println!("Found some other id: {}", id),
     }

}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

// Destructuring Enums and nested structs
//
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
//    ChangeColor(i32, i32, i32),
    ChangeColor(Color),
}

// Destructuring with patters is a convenient wy to use pieces of 
// values, such as the value from each field in a struct, separately
// from each other.

// Ignoring Values in a Pattern
//
// Ignoring an entire value with _
//
// Ignoring parts of a value with a Nested _
//
//let mut setting_value = Some(5);
//let new_setting_value = Some(10);
//
//match (setting_value, new setting_value) {
//    (Some(_), Some(_)) => {
//        println!("Cant overwrite an existing customized value");
//    }
//    _ => {
//        setting_value = new_setting_value;
//    }
//}
//
//println!("Setting is {:?}", setting_value);
//

// Ignoring an unsused variable by starting its name with a _
//
// Extra conditionals with match guards
//
// match guards are additional if conditions specified after the pattern
// in a match arm that must ALSO match, for that arm to be chosen. Like:
//match num {
//    Some(x) if x < 5 => println!("less than five: {}", x),
//    Some(x) => println!("{}", x),
//    None => (),
//}

// @ Bindings
//
// the at operator lets us create a variable that holds a value at the same
// time we're testing that value to see whether it matches a pattern.
//
enum MessageBind {
    Hello { id: i32 },
}
