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

    private static void readFile(String fileName, List<List<Integer>> reports) throws IOException {
        var reader = new FileReader(fileName);
        var tokenizer = new StreamTokenizer(reader);
        tokenizer.eolIsSignificant(true);
        int current = tokenizer.nextToken();

        var report = new ArrayList<Integer>();
        reports.add(report);

        while (current != StreamTokenizer.TT_EOF) {
            var foo = tokenizer.ttype;
            switch (current) {
                case StreamTokenizer.TT_NUMBER ->
                    report.add((int) tokenizer.nval);
                case StreamTokenizer.TT_EOL -> {
                    report = new ArrayList<>();
                    reports.add(report);
                }
            }
            current = tokenizer.nextToken();
        }

        if (reports.size() > 50)
            return;

        for (int i = 0; i < reports.size(); i++) {
            for (int j = 0; j < reports.get(i).size(); j++) {
                System.out.print(reports.get(i).get(j) + ", ");
            }
            System.out.println("");
        }
    }

    private static boolean isReportSafe(List<Integer> report) {
        if (report.isEmpty())
            return false;
        return Streams.zip(report.stream(), report.stream().skip(1), (a, b) -> a - b)
                        .collect(Collectors.teeing(
                Collectors.minBy(Integer::compareTo),
                Collectors.maxBy(Integer::compareTo),
                (min, max) -> {
                    if (max.get() < 0 && min.get() >= -3)
                        return true;
                    return min.get() > 0 && max.get() <= 3;
                }
        ));
    }

    public static void processReports1(List<List<Integer>> reports) {
        if (reports.size() < 50) {
            reports.stream()
                    .map(r -> isReportSafe(r) ? "Safe": "Unsafe")
                    .forEach(System.out::println);
        }

        var count = reports.stream().filter(Main::isReportSafe).count();
        System.out.println("Safe reports: " + count);
    }


    public static void main(String[] args) {
        String file = "src/main/resources/input";
//        String file = "src/main/resources/test.txt";
        try {
            var reports = new ArrayList<List<Integer>>();
            readFile(file, reports);
            processReports1(reports);
        } catch (Exception e) {
            System.err.println(e.getMessage());
            e.printStackTrace();
        }
    }
}