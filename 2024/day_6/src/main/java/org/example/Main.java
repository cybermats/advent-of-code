package org.example;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Main {

    private static String readFile(String fileName) throws IOException {
        return String.join("\n", Files.readAllLines(Path.of(fileName)));
    }


    public static void main(String[] args) {
        String file = "src/main/resources/input";
//        String file = "src/main/resources/test.txt";
        try {
            var content = readFile(file);
            System.out.println(content);

        } catch (Exception e) {
            System.err.println(e.getMessage());
            e.printStackTrace();
        }
    }
}