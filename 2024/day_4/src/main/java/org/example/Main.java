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

    private static List<List<Character>> restructureFile(String content) {
        var result = new ArrayList<List<Character>>();
        var row = new ArrayList<Character>();
        result.add(row);
        for(var i = 0; i < content.length(); i++) {
            char c = content.charAt(i);
            if(c == '\n') {
                row = new ArrayList<>();
                result.add(row);
                continue;
            }
            row.add(c);
        }
        if (row.size() == 0)
            result.removeLast();
        return result;
    }

    private record Point(int x, int y) {}
    private record Dimension(int width, int height) {}

    private static List<List<Point>> horisontals(Dimension dimension) {
        var points = IntStream.range(0, dimension.height)
                        .mapToObj(yStart ->
                                IntStream.range(0, dimension.width)
                                        .mapToObj(xStart -> new Point(xStart, yStart))
                                        .toList());
        return points.toList();
    }

    private static List<List<Point>> verticals(Dimension dimension) {
        var points = IntStream.range(0, dimension.width)
                .mapToObj(xStart ->
                        IntStream.range(0, dimension.height)
                                .mapToObj(yStart -> new Point(xStart, yStart))
                                .toList());
        return points.toList();
    }

    private static List<List<Point>> southEast(Dimension dimension) {
        var topRow = IntStream.range(0, dimension.width)
                .mapToObj(xStart ->
                        IntStream.range(0, dimension.height)
                .mapToObj(increase -> new Point(xStart + increase, increase))
                                .filter(point -> point.x < dimension.width && point.y < dimension.height)
                                .toList()
                );
        var leftColumn = IntStream.range(1, dimension.height)
                .mapToObj(yStart ->
                        IntStream.range(0, dimension.width)
                                .mapToObj(increase -> new Point(increase, yStart + increase))
                                .filter(point -> point.x < dimension.width && point.y < dimension.height)
                                .toList()
                );
        return Stream.concat(topRow, leftColumn).toList();
    }

    private static List<List<Point>> southWest(Dimension dimension) {
        var topRow = IntStream.range(0, dimension.width)
                .mapToObj(xStart ->
                        IntStream.range(0, dimension.height)
                                .mapToObj(increase -> new Point(xStart - increase, increase))
                                .filter(point -> point.x >= 0 )
                                .toList()
                );
        var leftColumn = IntStream.range(1, dimension.height)
                .mapToObj(yStart ->
                        IntStream.range(0, dimension.width)
                                .mapToObj(increase -> new Point(dimension.width - increase - 1, yStart + increase))
                                .filter(point -> point.x >= 0 && point.y < dimension.height)
                                .toList()
                );
        return Stream.concat(topRow, leftColumn).toList();
    }

    private static int wordCount(List<List<Character>> grid, List<Point> section, String pattern) {
        var patternCount = 0;
        var counter = 0;
        for (var p : section) {
            var c = grid.get(p.y).get(p.x);
            if (pattern.charAt(patternCount) == c) {
                patternCount++;
            } else if (patternCount > 0) {
                patternCount = 0;
                if (pattern.charAt(patternCount) == c) {
                    patternCount++;
                }
            }

            if (patternCount == pattern.length()) {
                counter++;
                patternCount = 0;
            }


        }
        return counter;
    }

    private static int wordsCount(List<List<Character>> grid, List<List<Point>> sections, String pattern) {
        var counter = 0;
        for(var section : sections) {
            counter += wordCount(grid, section, pattern);
            counter += wordCount(grid, section.reversed(), pattern);
        }
        return counter;
    }

    private static boolean hasCross(List<List<Character>> grid, Point point) {
        if (point.x < 1 || point.y < 1 || point.y >= grid.size() - 1 || point.x >= grid.get(point.y).size() - 1) {
            return false;
        }

        if (grid.get(point.y).get(point.x) != 'A') {
            return false;
        }

        var topLeft = grid.get(point.y-1).get(point.x-1);
        var bottomRight = grid.get(point.y+1).get(point.x+1);

        if ((topLeft == 'M' && bottomRight == 'S')||(topLeft == 'S' && bottomRight == 'M')) {
            var topRight = grid.get(point.y-1).get(point.x+1);
            var bottomLeft = grid.get(point.y+1).get(point.x-1);
            if ((topRight == 'M' && bottomLeft == 'S') || (topRight == 'S' && bottomLeft == 'M')) {
                return true;
            }
        }

        return false;
    }

    private static long countCrosses(List<List<Character>> grid) {
        var points = IntStream.range(0, grid.size())
                .mapToObj(y -> y)
                .flatMap(yStart ->
                        IntStream.range(0, grid.get(yStart).size())
                                .mapToObj(xStart ->new Point(xStart, yStart)))
                        .toList();
        var crosses = points.stream()
                .filter(point -> hasCross(grid, point))
                        .toList();
        return crosses.size();
    }


    public static void main(String[] args) {
        String file = "src/main/resources/input";
//        String file = "src/main/resources/test.txt";
        try {
            var content = readFile(file);
            System.out.println(content);
            var grid = restructureFile(content);
            System.out.println(grid);
            var dimension = new Dimension(grid.size(), grid.getFirst().size());
/*
            var points = horisontals(dimension);
            var counter = wordsCount(grid, points, "XMAS");
            points = verticals(dimension);
            counter += wordsCount(grid, points, "XMAS");
            points = southEast(dimension);
            counter += wordsCount(grid, points, "XMAS");
            points = southWest(dimension);
            counter += wordsCount(grid, points, "XMAS");
*/
            var counter = countCrosses(grid);

            System.out.println("Counter: " + counter);

        } catch (Exception e) {
            System.err.println(e.getMessage());
            e.printStackTrace();
        }
    }
}