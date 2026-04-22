#include<stdio.h>

int main() {
    long long n;
    if (scanf("%lld", &n) != 1) return 0;

    long long soma_esperada = (n * (n + 1)) / 2;
    long long soma_atual = 0;
    long long temp;

    for (int i = 0; i < n - 1; i++) {
        if (scanf("%lld", &temp) == 1) {
            soma_atual += temp;
        }
    }

    printf("%lld\n", soma_esperada - soma_atual);

    return 0;
}
