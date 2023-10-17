#include <uea/io.hpp>
#include <uea/string.hpp>

using namespace uea;

int main() {
    unsigned count = 0;

    uea::stdin.for_lines([&](std::string line) {
        auto pieces = split(' ', line);

        auto counts_raw = split('-', pieces[0]);
        unsigned min = atoi(counts_raw[0].c_str());
        unsigned max = atoi(counts_raw[1].c_str());

        char check = pieces[1][0];
        
        std::string text = pieces[2];

        unsigned found = 0;
        for (char c : text)
            if (c == check)
                found++;

        if (min <= found && found <= max)
            count++;
    });

    say("{}", count);
}
