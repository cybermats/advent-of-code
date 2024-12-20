package org.example;

import java.io.FileReader;
import java.io.IOException;
import java.io.StreamTokenizer;
import java.io.StringReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import com.google.common.collect.Streams;

public class Main {
    private record MulOperand(int a, int b) {
        public int value() {return a * b;}
    }

    private static String readFile(String fileName) throws IOException {
        return Files.readAllLines(Path.of(fileName)).stream().collect(Collectors.joining());
    }

    private static List<String> findAllInstructions(String data) {
        var pattern = Pattern.compile("mul\\(\\d{1,3},\\d{1,3}\\)");
        var matcher = pattern.matcher(data);

        var result = new ArrayList<String>();
        while(matcher.find()) {
            result.add(matcher.group());
        }
        return result;
    }

    private static MulOperand parseOperand(String data) throws Exception {
        var stringReader = new StringReader(data);
        var tokenizer = new StreamTokenizer(stringReader);
        var arguments = new ArrayList<Integer>();
        while(tokenizer.nextToken() != StreamTokenizer.TT_EOF) {
            switch (tokenizer.ttype){
                case StreamTokenizer.TT_WORD:
                    if(!tokenizer.sval.equals("mul")){throw new Exception("Invalid operand");}
                    break;
                case StreamTokenizer.TT_NUMBER:
                    arguments.add((int)tokenizer.nval);
                    break;
            }
        }
        if (arguments.size() != 2) {
            throw new Exception("Invalid number of arguments");
        }
        return new MulOperand(arguments.get(0), arguments.get(1));
    }

    public static List<MulOperand> parseInstructions(List<String> data) throws Exception {
        var result = new ArrayList<MulOperand>();
        for(int i = 0; i < data.size(); i++) {
            result.add(parseOperand(data.get(i)));
        }
        return result;
    }

    public static int executeInstructions(List<MulOperand> operands) throws Exception {
        var result = 0;
        for (var operand : operands) {
            var product = operand.value();
            result += product;
        }
        return result;
    }


    public static void main(String[] args) {
        String file = "src/main/resources/input";
//        String file = "src/main/resources/test.txt";
        try {
            var content = readFile(file);
            System.out.println(content);
            var instructions = findAllInstructions(content);
            System.out.println(instructions);
            var operands = parseInstructions(instructions);
            System.out.println(operands);
            var sum = executeInstructions(operands);
            System.out.println(sum);
        } catch (Exception e) {
            System.err.println(e.getMessage());
            e.printStackTrace();
        }
    }
}