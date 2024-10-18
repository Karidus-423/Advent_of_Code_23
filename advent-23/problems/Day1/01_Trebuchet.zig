const std = @import("std");
const print = std.debug.print;
const inputSize = 21591;
const Allocator = std.mem.Allocator;

const calibrationValues = enum(u8) {
    zero = '0',
    one = '1',
    two = '2',
    three = '3',
    four = '4',
    five = '5',
    six = '6',
    seven = '7',
    eight = '8',
    nine = '9',
};

pub fn main() !void {
    //Standard opening and closing file setup.
    const cwd = std.fs.cwd();
    //Tries to open the directory.
    var inputDir = try cwd.openDir("./problems/Day1", .{});
    defer inputDir.close();
    //Tries to open the file.
    const inputFile = try inputDir.openFile("input.txt", .{});
    defer inputFile.close();

    var inputText: [inputSize]u8 = undefined;
    _ = try std.fs.File.readAll(inputFile, &inputText);

    calibration_solver(inputText);
}

fn calibration_solver(problemInput: [inputSize]u8) void {
    //Separates inputText buffer into a slice per line.
    var line = std.mem.tokenizeAny(u8, &problemInput, "\n");
    var lineNumber: usize = 0;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    //We travel between the slices.
    while (line.next()) |lineContent| {
        lineNumber += 1;
        var numberList = std.ArrayList(u8).init(allocator);

        if (lineNumber == 1) {
            print("{s}\n", .{lineContent});

            for (lineContent) |_| {
                const n = try std.fmt.parseInt(u8, lineContent, 10);
                try numberList.append(n);
            }

            for (numberList) |value| {
                print("Scanned:{d}", .{numberList[value]});
            }
        }
    }
}
