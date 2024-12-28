package org.example;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Main {

    private static void readFile(String fileName, Map<Integer, Set<Integer>> rules, List<List<Integer>> updates) throws Exception {
        var lines = Files.readAllLines(Path.of(fileName));
        var parsingRules = true;
        for (var line : lines) {
            if (parsingRules) {
                if (line.trim().isEmpty()) {
                    parsingRules = false;
                    continue;
                }
                var rule = line.trim().split("\\|");
                if (rule.length != 2) {
                    throw new Exception("Invalid format of Page Ordering Rule");
                }

                var start = Integer.parseInt(rule[0]);
                var end = Integer.parseInt(rule[1]);
                rules.putIfAbsent(start, new HashSet<>());
                rules.get(start).add(end);
            }
            else {
                if (line.trim().isEmpty()) {
                    throw new Exception("No rules are allowed during Update listing");
                }
                var update =
                        Arrays.stream(line.trim().split(","))
                        .map(Integer::parseInt)
                                .toList();
                updates.add(update);
            }
        }
    }

    private static boolean processUpdate(Map<Integer, Set<Integer>> rules, List<Integer> update) {
        var seenPages = new HashSet<Integer>();
        for (var page : update) {
            var rule = rules.get(page);
            if (rule != null) {
                var intersection = new HashSet<>(seenPages);
                intersection.retainAll(rule);
                if (!intersection.isEmpty()) {
                    return false;
                }
            }
            seenPages.add(page);
        }
        return true;
    }

    private static List<List<Integer>> findPrintableUpdates(Map<Integer, Set<Integer>> rules, List<List<Integer>> updates) {
        return
                updates.stream()
                        .filter(update -> processUpdate(rules, update))
                        .toList();
    }

    private static List<Integer> findMiddlePageNumbers(List<List<Integer>> updates) {
        return updates.stream()
                .map(update -> update.get(update.size()/2))
                .toList();
    }

    public static void main(String[] args) {
        String file = "src/main/resources/input";
//        String file = "src/main/resources/test.txt";
        try {
            var rules = new HashMap<Integer, Set<Integer>>();
            var updates = new ArrayList<List<Integer>>();
            readFile(file, rules, updates);
            System.out.println("Rules: " + rules);
            System.out.println("Updates: " + updates);
            var printableUpdates = findPrintableUpdates(rules, updates);
            System.out.println("Printable Updates: " + printableUpdates);
            var middlePageNumbers = findMiddlePageNumbers(printableUpdates);
            System.out.println("Middle Page Numbers: " + middlePageNumbers);
            var sum = middlePageNumbers.stream().mapToInt(Integer::intValue).sum();
            System.out.println("Sum of Middle Page Numbers: " + sum);

        } catch (Exception e) {
            System.err.println(e.getMessage());
            e.printStackTrace();
        }
    }
}