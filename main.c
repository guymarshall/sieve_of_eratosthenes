#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{
    if (argc != 2)
    {
        printf("Usage: %s <limit>\n", argv[0]);
        return 1;
    }

    int limit = atoi(argv[1]);
    if (limit < 2)
    {
        printf("Please provide a limit greater than or equal to 2.\n");
        return 1;
    }

    int is_prime[limit + 1];

    for (int i = 0; i <= limit; i++)
    {
        is_prime[i] = 1;
    }

    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i * i <= limit; i++)
    {
        if (is_prime[i])
        {
            for (int multiple = i * i; multiple <= limit; multiple += i)
            {
                is_prime[multiple] = 0;
            }
        }
    }

    int prime_count = 0;
    for (int i = 2; i <= limit; i++)
    {
        if (is_prime[i])
        {
            prime_count++;
        }
    }

    printf("Number of primes up to %d: %d\n", limit, prime_count);

    return 0;
}