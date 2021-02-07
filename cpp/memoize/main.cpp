#include <iostream>

#include <map>
#include <utility>

// helpers
int sum(int i, int j) { std::cout << "Calculating sum for: " << i << " + " << j << std::endl; return i + j; }
int succ(int i) { std::cout << "Calculating succ for: " << i << std::endl; return i + 1; }
double succDouble(int i) { std::cout << "Caluating succDouble for: " << i << std:: endl; return static_cast<double>(i+1)+0.5; }

// TODO: add perfect forwarding
// TODO: do with single function

// memoize_func : caches results per function per inputs
template <class F, class ...Args>
auto memoize_func(F f, Args... args)
{
    std::tuple<Args...> myTuple(args...);
    static std::map<std::tuple<Args...>, decltype(f(args...))> results;
    if (results.find(myTuple) != results.end())
    {
        return results[myTuple];
    }
    results[myTuple] = f(args...);
    return results[myTuple];
}

// returns memoized function
template <class F>
auto memoize(F f)
{
    auto mem_f = [f](auto... args) { return memoize_func(f,args...); };
    return mem_f;
}

void test_memoize();
void test_memoize_func();

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

void test_memoize_func()
{
    std::cout << memoize_func(sum, 0,2) << std::endl;
    std::cout << memoize_func(sum, 0,2) << std::endl;
    std::cout << memoize_func(sum, 0,2) << std::endl;
    std::cout << memoize_func(sum, 0,3) << std::endl;
    std::cout << memoize_func(sum, 0,3) << std::endl;
    std::cout << memoize_func(sum, 0,3) << std::endl;

    std::cout << memoize_func(succ,0) <<std::endl;
    std::cout << memoize_func(succ,0) <<std::endl;
    std::cout << memoize_func(succ,0) <<std::endl;
}


