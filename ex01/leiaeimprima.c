#include <stdio.h>

int main() {
    long long int a; char b;
    if (scanf("%lld %c", &a, &b) == 2) {
        printf("%lld\n%c", a, b);
    }

    return 0;
}