
#include <iostream>
#include <array>
#include <algorithm>
#include <iterator>

class Cell
{
public:
    Cell(int value) { value_ = value; }
    void setValue(int value) {value_ = value;}
    int value() { return value_; }
private:
    int value_;
};

// inner stack collection you want to change
std::array<Cell, 3> someArray = { Cell(1), Cell(2), Cell(3) };

// function used to access collection without making a copy
std::array<Cell, 3> &otherArray()
{
    return someArray;
}

int main()
{

    // update inner collection
    int i = 0;
    std::array<int, 3> other = { 5, 6, 7 };
    std::for_each(std::begin(other), std::end(other), [&](const int &n){
            otherArray()[i].setValue(n);
            i++;
    });

    // verify change occurred on inner strucuture
    std::cout << someArray[0].value() << std::endl;
    std::cout << someArray[1].value() << std::endl;
    std::cout << someArray[2].value() << std::endl;
    return 0;
}
