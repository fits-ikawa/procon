#include <iostream>
#include <vector>
#include <algorithm>

int main()
{
    int n;
    std::cin >> n;

    std::vector<int> a(n);
    for (int i = 0; i < n; ++i)
    {
        std::cin >> a[i];
    }

    int count = 0;

    while (std::all_of(a.begin(), a.end(), [](int x)
                       { return x % 2 == 0; }))
    {
        for (int i = 0; i < n; ++i)
        {
            a[i] /= 2;
        }
        ++count;
    }

    std::cout << count << std::endl;
}
