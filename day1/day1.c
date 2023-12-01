#include <assert.h>
#include <stdbool.h>
#include <stdio.h>

bool is_digit(const char c) {
    return c >= '0' && c <= '9';
}

bool is_newline(const char c) {
    return c == '\n';
}

short to_number(char decs, char units) {
    assert(is_digit(decs) && is_digit(units));
    decs -= '0';
    units -= '0';
    return decs * 10 + units;
}

__uint64_t solve(const char* filename) {
    FILE* file = fopen(filename, "r");

    __uint64_t sum = 0;

    char current_char, first_digit = 0, last_digit = 0;
    while ((current_char = fgetc(file)) != EOF) {
        if (is_newline(current_char)) {
            sum += to_number(first_digit, last_digit);
            first_digit = 0;
            last_digit = 0;
            continue;
        }

        if (!is_digit(current_char)) {
            continue;
        }

        if (first_digit == 0) {
            first_digit = current_char;
        }
        last_digit = current_char;
    }

    return sum;
}

int main() {
    const char* input_filename = "input.txt";
    const __int64_t solution = solve(input_filename);
    printf("Solution: %ld\n", solution);
}
