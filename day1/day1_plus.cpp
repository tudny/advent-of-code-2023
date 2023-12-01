#include <iostream>
#include <fstream>
#include <map>
#include <algorithm>

std::map<std::string, int64_t> allowed_names_normal = {
    {"one", 1},
    {"two", 2},
    {"three", 3},
    {"four", 4},
    {"five", 5},
    {"six", 6},
    {"seven", 7},
    {"eight", 8},
    {"nine", 9}
};

std::map<std::string, int64_t> allowed_names_reversed = {
    {"eno", 1},
    {"owt", 2},
    {"eerht", 3},
    {"ruof", 4},
    {"evif", 5},
    {"xis", 6},
    {"neves", 7},
    {"thgie", 8},
    {"enin", 9}
};

int64_t find_first_digits(const std::string& str, const std::map<std::string, int64_t>& allowed_names) {
    std::map<size_t, int64_t> saved;
    for (const auto& [name, value]: allowed_names) {
        if (auto pos = str.find(name); pos != std::string::npos) {
            saved[pos] = value;
        }
    }

    for (char it = '0'; it <= '9'; ++it) {
        if (auto pos = str.find(it); pos != std::string::npos) {
            saved[pos] = it - '0';
        }
    }

    return saved.begin()->second;
}

int64_t solve(const std::string& filename) {
    std::ifstream file(filename);
    std::string str;

    int64_t sum = 0;
    while (std::getline(file, str)) {
        const auto first_digit = find_first_digits(str, allowed_names_normal);
        std::reverse(str.begin(), str.end());
        const auto last_digit = find_first_digits(str, allowed_names_reversed);
        sum += first_digit * 10 + last_digit;
    }

    return sum;
}

int main() {
    const std::string input_filename = "input.txt";
    const int64_t solution = solve(input_filename);
    std::cout << "Solution: " << solution << std::endl;
}
