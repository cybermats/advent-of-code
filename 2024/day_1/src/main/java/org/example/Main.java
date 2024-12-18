package org.example;

import java.io.FileReader;
import java.io.IOException;
import java.io.StreamTokenizer;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

import com.google.common.collect.Streams;

public class Main {

    private static void readFile(String fileName, List<Integer> leftList, List<Integer> rightList) throws IOException {
        var reader = new FileReader(fileName);
        var tokenizer = new StreamTokenizer(reader);
        int current = tokenizer.nextToken();
        var left = true;

        while (current != StreamTokenizer.TT_EOF) {
            if (tokenizer.ttype == StreamTokenizer.TT_NUMBER) {
                var list = left ? leftList : rightList;
                list.add((int) tokenizer.nval);
            }
            current = tokenizer.nextToken();
            left = !left;
        }

        if (!left) {
            System.err.println("Invalid format of input file");
            return;
        }

        if (leftList.size() > 50)
            return;

        for (int i = 0; i < leftList.size(); i++) {
            System.out.print(leftList.get(i));
            System.out.print(", ");
            System.out.println(rightList.get(i));
        }
    }

    private static void processListsPart1(List<Integer> leftList, List<Integer> rightList) {
        assert leftList.size() == rightList.size();

        Collections.sort(leftList);
        Collections.sort(rightList);

        var sum = Streams
                .zip(leftList.stream(), rightList.stream(), (left, right) -> Math.abs(left - right))
                .collect(Collectors.summarizingInt(Integer::intValue));
        System.out.println("Sum is: " + sum.getSum());

    }

    public static void main(String[] args) {
        String file = "src/main/resources/aoc_day1input.txt";
        try {
            var leftList = new ArrayList<Integer>();
            var rightList = new ArrayList<Integer>();
            readFile(file, leftList, rightList);
            processListsPart1(leftList, rightList);
        } catch (Exception e) {
            System.err.println(e.getMessage());
            e.printStackTrace();
        }
    }
}