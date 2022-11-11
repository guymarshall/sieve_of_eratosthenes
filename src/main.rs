fn main() {
    println!("Hello, world!");
}

/*
package com.marshall.guy;

import java.util.TreeSet;
import java.util.Scanner;

public class Main2 {

    public static TreeSet<Integer> generateTreeSet(int number)
    {
        TreeSet<Integer> numbers = new TreeSet<>();

        for (int i = 2; i <= number; i++)
        {
            numbers.add(i);
        }

        return numbers;
    }

    public static void main(String[] args)
    {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter a positive integer: ");
        int userInput = scanner.nextInt();
        scanner.close();

        TreeSet<Integer> numbers = generateTreeSet(userInput);

        int ceilingRoot = (int) Math.ceil(Math.sqrt(userInput));
        for (int i = 2; i <= ceilingRoot; i++)
        {
            for (int j = i; j <= userInput; j++)
            {
                numbers.remove(Integer.valueOf(i * j));
            }
        }
        
        System.out.println(numbers);
    }
}
*/