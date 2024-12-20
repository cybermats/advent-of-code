package org.example;

import java.io.FileReader;
import java.io.IOException;
import java.io.StreamTokenizer;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import com.google.common.collect.Streams;

public class Main {

    private static void readFile(String fileName) throws IOException {
        var reader = new FileReader(fileName);
        reader.close();
    }


    public static void main(String[] args) {
//        String file = "src/main/resources/input";
        String file = "src/main/resources/test.txt";
        try {
            readFile(file);
        } catch (Exception e) {
            System.err.println(e.getMessage());
            e.printStackTrace();
        }
    }
}