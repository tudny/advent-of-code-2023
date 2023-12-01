#include <iostream>
#include <fstream>
#include <map>
#include <algorithm>
#include <optional>

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

std::map<std::string, int64_t> allowed_names_reversed = [] {
    std::map<std::string, int64_t> reversed;
    for (const auto& [name, value]: allowed_names_normal) {
        reversed[std::string(name.rbegin(), name.rend())] = value;
    }
    return reversed;
}();

std::optional<int64_t> is_digit(const char& c) {
    if (c >= '0' && c <= '9') {
        return c - '0';
    }
    return {};
}

std::optional<int64_t> is_letterized_digit(
    const std::string& str,
    const size_t idx,
    const std::map<std::string, int64_t>& allowed_names
) {
    for (const auto& [name, value]: allowed_names) {
        if (str.substr(idx, name.size()) == name) {
            return value;
        }
    }
    return {};
}

int64_t find_first_digits(const std::string& str, const std::map<std::string, int64_t>& allowed_names) {
    for (size_t idx = 0; idx < str.size(); ++idx) {
        if (const auto digit = is_digit(str[idx])) {
            return *digit;
        }
        if (const auto digit = is_letterized_digit(str, idx, allowed_names)) {
            return *digit;
        }
    }
    return 0;
}

int64_t solve(const std::string& filename) {
    std::ifstream file(filename);
    std::string str;

    int64_t sum = 0;
    while (std::getline(file, str)) {
        const auto first_digit = find_first_digits(str, allowed_names_normal);
        std::ranges::reverse(str);
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
