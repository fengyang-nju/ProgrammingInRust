#include <iostream>
#include <vector>

int main() {
    auto v1 = std::vector<int>{11, 22, 33};
    const auto v2 = v1;
    const auto v3 = std::move(v1);
    std::cout << v1.size() << " " << v2.size() << " " << v3.size();
}