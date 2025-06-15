#include <stdio.h>

#define LIMIT 1000000

int main(int argc, char *argv[])
{
    int is_prime[LIMIT + 1];

    for (int i = 0; i <= LIMIT; i++)
    {
        is_prime[i] = 1;
    }

    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i * i <= LIMIT; i++)
    {
        if (is_prime[i])
        {
            for (int multiple = i * i; multiple <= LIMIT; multiple += i)
            {
                is_prime[multiple] = 0;
            }
        }
    }

    int prime_count = 0;
    for (int i = 2; i <= LIMIT; i++)
    {
        if (is_prime[i])
        {
            prime_count++;
        }
    }

    printf("Number of primes up to %d: %d\n", LIMIT, prime_count);

    return 0;
}