#include <iostream>

#include <map>
#include <utility>

// helpers
int sum(int i, int j) { std::cout << "Calculating sum for: " << i << " + " << j << std::endl; return i + j; }
int succ(int i) { std::cout << "Calculating succ for: " << i << std::endl; return i + 1; }
double succDouble(int i) { std::cout << "Caluating succDouble for: " << i << std:: endl; return static_cast<double>(i+1)+0.5; }

// concrete
int succ_mem(int i)
{
    static std::map<int, int>results;

    if (results.find(i) != results.end())
    {
        return results[i];
    }

    results[i] = succ(i);
    return results[i];
}

//template <class T>
//T memoize(T func)
//{
//    auto func_mem = [func](auto args...){
//        static std::map<auto, auto> results;
//
//        if (results.find(args...) != results.end())
//        {
//            return results[args...];
//        }
//        results[args...] = func(args...);
//        return results;
//
//    };
//
//    return func_mem;
//}

// memoize_func : caches results per function per inputs
// currently requires explicit typing
template <class F, class R, class ...Args>
R memoize_func(F f, Args... args)
{
    std::tuple<Args...> myTuple(args...);
    static std::map<std::tuple<Args...>, R> results;
    if (results.find(myTuple) != results.end())
    {
        return results[myTuple];
    }
    results[myTuple] = f(args...);
    return results[myTuple];
}


int main(int argc, char *argv[])
{
    std::cout << memoize_func<decltype(sum), int, int>(sum, 1,2) << std::endl;
    std::cout << memoize_func<decltype(sum), int, int>(sum, 1,2) << std::endl;
    std::cout << 1 << " -> " << memoize_func<decltype(succ),int,int>(succ,1) << std::endl;
    std::cout << 1 << " -> " << memoize_func<decltype(succ),int,int>(succ,1) << std::endl;
    std::cout << 1 << " -> " << memoize_func<decltype(succDouble),double, int>(succDouble,1) << std::endl;

    //memoise_func(succ,1);
    return 0;
}



