
#include <iostream>
#include <array>
#include <memory>

class Cell
{
public:
    Cell(int value) { value_ = value; }
    int value_;
};

int main()
{
    std::array<std::shared_ptr<Cell>, 3> arr;
    arr[0] = std::shared_ptr<Cell>(new Cell(1));
    arr[1] = std::shared_ptr<Cell>(new Cell(2));
    arr[2] = std::shared_ptr<Cell>(new Cell(3));

    std::array<std::array<std::shared_ptr<Cell>, 3>, 3> matrix;

    for (auto m : matrix)
    {
        for (auto x : m)
        {
            std::cout << x->value_ << std::endl;
        }
    }
    
    return 0;
}
