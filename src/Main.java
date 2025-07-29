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

        boolean[] is_prime = new boolean[limit + 1];
        for (int i = 0; i <= limit; i++) {
            is_prime[i] = true;
        }
        is_prime[0] = false;
        is_prime[1] = false;

        for (int i = 2; i < limit; i++) {
            if (i * i > limit) {
                break;
            }
            if (is_prime[i]) {
                int multiple = i * i;
                while (multiple <= limit) {
                    is_prime[multiple] = false;
                    multiple += i;
                }
            }
        }

        int prime_count = 0;
        for (int i = 0; i < limit + 1; i++) {
            if (is_prime[i]) {
                prime_count++;
            }
        }

        System.out.printf("Number of primes up to %d: %d%n", limit, prime_count);
    }
}
