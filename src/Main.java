import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.println("Limit:");
        int limit = scanner.nextInt();

        if (limit < 2) {
            System.out.println("Please provide a limit greater than or equal to 2.");
            main(args);
        }

        ArrayList<Boolean> is_prime = new ArrayList<>(limit + 1);
        for (int i = 0; i <= limit; i++) {
            is_prime.add(true);
        }
        is_prime.set(0, false);
        is_prime.set(1, false);

        for (int i = 2; i < limit; i++) {
            if (i * i > limit) {
                break;
            }
            if (is_prime.get(i)) {
                int multiple = i * i;
                while (multiple <= limit) {
                    is_prime.set(multiple, false);
                    multiple += i;
                }
            }
        }

        long prime_count = is_prime.stream().filter(i -> i).count();
        System.out.printf("Number of primes up to %d: %d%n", limit, prime_count);
    }
}
