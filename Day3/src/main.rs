use std::io;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() -> io::Result<()> {
    let mut points: i32 = 0;
    let input = std::fs::read_to_string("input.txt")?;
    let mut count = 0;
    let mut one = "";
    let mut two = "";
    let mut three = "";
    for line in input.lines() {
        match count {
            0 => {
                one = line;
                count = 1;
            }
            1 => {
                two = line;
                count = 2;
            }
            2 => {
                three = line;
                count = 0;
                let vec1: Vec<&str> = one.split("").collect();
                let vec2: Vec<&str> = two.split("").collect();
                let vec3: Vec<&str> = three.split("").collect();
                for a in vec1 {
                    if (vec2.contains(&a) && vec3.contains(&a)) && a != "" {
                        points += priority_calc(a);
                        break;
                    }
                }
            }
            3 => {}
            _ => panic!("This will never happen."),
        }
    }
    println!("{}", points);

    Ok(())
}

fn priority_calc(string: &str) -> i32 {
    let charn: Vec<char> = string.chars().collect();
    if ASCII_LOWER.contains(&charn[0].to_ascii_lowercase()) {
        let position = ASCII_LOWER
            .iter()
            .position(|&x| x == charn[0].to_ascii_lowercase())
            .unwrap();
        match charn[0].is_lowercase() {
            true => (position + 1) as i32,
            false => 26 + (position + 1) as i32,
        }
    } else {
        panic!("This should NOT happen!")
    }
}
