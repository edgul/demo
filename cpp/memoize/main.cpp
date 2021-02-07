#include <iostream>

#include <map>

// helpers
int sum(int i, int j) { std::cout << "Calculating sum for: " << i << " + " << j << std::endl; return i + j; }
int succ(int i) { std::cout << "Calculating succ for: " << i << std::endl; return i + 1; }
double succDouble(int i) { std::cout << "Caluating succDouble for: " << i << std:: endl; return static_cast<double>(i+1)+0.5; }

// TODO: add perfect forwarding

// #1 memoise : returns memoized version of argument function
template <class F > auto memoize(F f)
{
    auto mem_f = [f](auto... args) { 
        std::tuple<decltype(args)...> myTuple(args...); // tuple to pass into std::maps template param (might be better way)
        static std::map<std::tuple<decltype(args)...>, decltype(f(args...))> results; // map: input -> f(input)
        if (results.find(myTuple) != results.end())
        {
            return results[myTuple];
        }
        results[myTuple] = f(args...);
        return results[myTuple];
    };
    return mem_f;
}



void test_memoize();

int main(int argc, char *argv[])
{
    test_memoize();
    return 0;
}

void test_memoize()
{
    auto memoizedSucc = memoize(succ);
    std::cout << memoizedSucc(5) << std::endl;
    std::cout << memoizedSucc(5) << std::endl;
    std::cout << memoizedSucc(5) << std::endl;
   
    auto memoizedSum = memoize(sum);
    std::cout << memoizedSum(5,6) << std::endl;
    std::cout << memoizedSum(5,6) << std::endl;
    std::cout << memoizedSum(1,6) << std::endl;
    std::cout << memoizedSum(1,6) << std::endl;
}


