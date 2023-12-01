const std = @import("std");

pub fn main() !void {
    // var file = try std.fs.cwd().openFile("input", .{}); // 54951
    // var file = try std.fs.cwd().openFile("easy_input_part_one", .{}); // 142
    var file = try std.fs.cwd().openFile("easy_input_part_two", .{}); // 209, 281
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    // max line in input file has 51 ASCII characters (+ \n)
    var buf: [52]u8 = undefined;
    var total: u32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var n: u8 = 0;
        var first = true;
        var last: u8 = 0;
        for (line) |char| {
            if (char >= 49 and char <= 57) {
                if (first) {
                    n += (char - 48) * 10;
                    first = false;
                }

                last = char - 48;
            }
        }
        n += last;
        // std.debug.print("n: {d}\n", .{n});
        total += n;
    }
    std.debug.print("{d}\n", .{total});
}
