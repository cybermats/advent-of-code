package org.example;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Main {
    private static class Day5Comparator implements Comparator<Integer> {
        private final Map<Integer, Set<Integer>> _lessThan;

        public Day5Comparator(Map<Integer, Set<Integer>> lessThan) {
            _lessThan = lessThan;
        }


        @Override
        public int compare(Integer o1, Integer o2) {
            if (o1.equals(o2))
                return 0;
            var ruleA = _lessThan.get(o1);
            if (ruleA != null && ruleA.contains(o2))
                return -1;
            var ruleB = _lessThan.get(o2);
            if (ruleB != null && ruleB.contains(o1))
                return 1;
            return 0;
        }
    }



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

    private static List<List<Integer>> findUnPrintableUpdates(Map<Integer, Set<Integer>> rules, List<List<Integer>> updates) {
        return
                updates.stream()
                        .filter(update -> !processUpdate(rules, update))
                        .toList();
    }

    private static record Link(Integer head, Optional<Link> tail){}

    private static List<Integer> reorderUpdate(Map<Integer, Set<Integer>> rules, List<Integer> update) throws Exception {
        var comparator = new Day5Comparator(rules);
        var result = new ArrayList<>(update);
        result.sort(comparator);
        return result;
    }

    private static List<List<Integer>> reorderAllUpdates(Map<Integer, Set<Integer>> rules, List<List<Integer>> updates) throws Exception {
        var result = new ArrayList<List<Integer>>();
        for (var update: updates) {
            var ordered = reorderUpdate(rules, update);
            result.add(ordered);
        }
        return result;
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

            var unprintableUpdates = findUnPrintableUpdates(rules, updates);
            System.out.println("Unprintable updates: " + unprintableUpdates);
            var reorderUpdates = reorderAllUpdates(rules, unprintableUpdates);
            System.out.println("Reorder updates: " + reorderUpdates);

            var middlePageNumbers = findMiddlePageNumbers(reorderUpdates);
            System.out.println("Middle page numbers: " + middlePageNumbers);
            var sum = middlePageNumbers.stream().mapToInt(Integer::intValue).sum();
            System.out.println("Sum: " + sum);


        } catch (Exception e) {
            System.err.println(e.getMessage());
            e.printStackTrace();
        }
    }
}