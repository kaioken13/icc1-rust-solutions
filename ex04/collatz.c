#include <stdio.h>

int main() {
    int a;
    int n = 1;

    scanf("%d", &a);

    while (a != 1) {
        printf("%d ", a);

        if (a % 2 == 0) {
            a = a / 2;
        } else {
            a = a * 3 + 1;
        }

        n++;
    }

    printf("%d\n", a);
    printf("%d", n);
    return 0;
}