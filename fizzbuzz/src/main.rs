const FROM: usize = 1;
const TO: usize = 100;
const EMPTY_STRING: String = String::new();

fn main() {
    let mut output: [String; TO as usize] = [EMPTY_STRING; TO];

    for i in FROM..TO {
        let mut divisible = false;
        if i % 3 == 0 {
            output[i].push_str("Fizz");
            divisible = true;
        }
        if i % 5 == 0 {
            output[i].push_str("Buzz");
            divisible = true;
        }
        if !divisible {
            output[i].push_str(i.to_string().as_str());
        }

        println!("{i}: {}", output[i]);
    }
}
