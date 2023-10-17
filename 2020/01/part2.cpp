#include <iostream>
#include <vector>

int main() {
    std::vector<int> numbers;

    {
        int tmp;
        while(std::cin >> tmp)
            numbers.push_back(tmp);
    }

    for (int i = 0; i < numbers.size(); ++i)
        for (int j = i+1; j < numbers.size(); ++j)
            for (int k = j+1; k < numbers.size(); ++k)
                if ((numbers[i] + numbers[j] + numbers[k]) == 2020) {
                    std::cout << (numbers[i] * numbers[j] * numbers[k]) << '\n';
                    return 0;
                }

    return 1;
}
