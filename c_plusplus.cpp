#include <string>
#include <iostream>

int main() {
    std::string name;
    std::cout << "Hello, what's your name?" << std::endl << "> ";
    std::cin >> name;
    std::cout << "Nice to meet you, " << name << "." << std::endl;
    return 0;
}
