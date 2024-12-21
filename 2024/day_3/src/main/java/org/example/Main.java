package org.example;

import java.io.IOException;
import java.io.StreamTokenizer;
import java.io.StringReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Main {
    private static final class OperandState {
        private int sum = 0;
        private boolean active = true;

        private OperandState() {
        }

        public void sum(int sum) {
            if (active)
                this.sum += sum;
        }
        public int sum() {
            return sum;
        }
        public void active(boolean active) {
            this.active = active;
        }

    }
    private static abstract class Operand {
        public abstract void Operate(OperandState state);
    }
    private static class MulOperand extends Operand {
        private int a, b;
        public MulOperand(int a, int b) {
            this.a = a;
            this.b = b;
        }
        public void Operate(OperandState state) {
            state.sum(a * b);
        }
        public String toString() {
            return "Mul(" + a + ", " + b + ")";
        }
    }
    private static class DoOperand extends Operand {
        private boolean active;
        public DoOperand(boolean active) {
            this.active = active;
        }
        public void Operate(OperandState state) {
            state.active(active);
        }
        public String toString() {
            return active ? "Do()" : "Don't()";
        }
    }

    private static String readFile(String fileName) throws IOException {
        return Files.readAllLines(Path.of(fileName)).stream().collect(Collectors.joining());
    }

    private static List<String> findAllInstructions(String data) {
        var pattern = Pattern.compile("(mul\\(\\d{1,3},\\d{1,3}\\))|(do\\(\\))|(don't\\(\\))");
        var matcher = pattern.matcher(data);

        var result = new ArrayList<String>();
        while(matcher.find()) {
            result.add(matcher.group());
        }
        return result;
    }

    private static Operand parseOperand(String data) throws Exception {
        var stringReader = new StringReader(data);
        var tokenizer = new StreamTokenizer(stringReader);
        String type = "";
        var arguments = new ArrayList<Integer>();

        while(tokenizer.nextToken() != StreamTokenizer.TT_EOF) {
            switch (tokenizer.ttype){
                case StreamTokenizer.TT_WORD:
                    type = tokenizer.sval;
                    break;
                case StreamTokenizer.TT_NUMBER:
                    arguments.add((int)tokenizer.nval);
                    break;
                default:
                    break;
            }
        }

        switch (type) {
            case "mul":
                if (arguments.size() != 2) {
                    throw new Exception("Invalid number of arguments");
                }
                return new MulOperand(arguments.get(0), arguments.get(1));
            case "do":
                if (arguments.size() != 0) {
                    throw new Exception("Invalid number of arguments");
                }
                return new DoOperand(true);
            case "don":
                if (arguments.size() != 0) {
                    throw new Exception("Invalid number of arguments");
                }
                return new DoOperand(false);
            default:
                throw new Exception("Unknown operand type: " + type);
        }

    }

    public static List<Operand> parseInstructions(List<String> data) throws Exception {
        var result = new ArrayList<Operand>();
        for(int i = 0; i < data.size(); i++) {
            result.add(parseOperand(data.get(i)));
        }
        return result;
    }

    public static int executeInstructions(List<Operand> operands) throws Exception {
        var state = new OperandState();
        for (var operand : operands) {
            operand.Operate(state);
        }
        return state.sum();
    }


    public static void main(String[] args) {
        String file = "src/main/resources/input";
//        String file = "src/main/resources/test.txt";
//        String file = "src/main/resources/test2.txt";
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