fn generate_vec(number: i32) -> Vec<i32> {
    (2..=number).collect()
}

pub fn is_prime_number(number: i32) -> bool {
    if number % 2 == 0 {
        return false;
    }
    let root_of_number: i32 = (number as f64).sqrt() as i32;
    for i in (3..root_of_number).step_by(2) {
        if number % i == 0 {
            return false
        }
    }

    return true;
}

fn main() {
    let user_input: i32 = 100;
}

/*
public static void main(String[] args) {
    int ceilingRoot = (int) Math.ceil(Math.sqrt(userInput));
    for (int i = 2; i <= ceilingRoot; i++) {
        for (int j = i; j <= userInput; j++) {
            numbers.remove(Integer.valueOf(i * j));
        }
    }
    
    System.out.println(numbers);
}
*/