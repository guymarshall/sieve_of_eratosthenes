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

        boolean[] isPrime = new boolean[limit + 1];
        for (int i = 0; i <= limit; i++) {
            isPrime[i] = true;
        }
        isPrime[0] = false;
        isPrime[1] = false;

        for (int i = 2; i < limit; i++) {
            if (i * i > limit) {
                break;
            }
            if (isPrime[i]) {
                int multiple = i * i;
                while (multiple <= limit) {
                    isPrime[multiple] = false;
                    multiple += i;
                }
            }
        }

        int primeCount = 0;
        for (int i = 0; i < limit + 1; i++) {
            if (isPrime[i]) {
                primeCount++;
            }
        }

        System.out.printf("Number of primes up to %d: %d%n", limit, primeCount);
    }
}
