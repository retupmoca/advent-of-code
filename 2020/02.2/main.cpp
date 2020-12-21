#include <uea/io.hpp>
#include <uea/string.hpp>

using namespace uea;

int main() {
    unsigned count = 0;

    uea::stdin.for_lines([&](std::string line) {
        auto pieces = split(' ', line);

        auto counts_raw = split('-', pieces[0]);
        unsigned a = atoi(counts_raw[0].c_str()) - 1;
        unsigned b = atoi(counts_raw[1].c_str()) - 1;

        char check = pieces[1][0];
        
        std::string text = pieces[2];

        if(text[a] != text[b] && (text[a] == check || text[b] == check))
            count++;
    });

    say("{}", count);
}
