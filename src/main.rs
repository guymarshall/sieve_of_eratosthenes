fn generate_vec(number: i32) -> Vec<i32> {
    (2..=number).collect()
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