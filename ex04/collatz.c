#include <stdio.h>

int main() {
    unsigned int a;
    unsigned int n = 1;

    scanf("%u", &a);

    while (a != 1) {
        printf("%u ", a);

        if (a % 2 == 0) {
            a = a / 2;
        } else {
            a = a * 3 + 1;
        }

        n++;
    }

    printf("%u\n", a);
    printf("%u", n);
    return 0;
}