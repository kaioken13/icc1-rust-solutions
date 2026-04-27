#include <stdio.h>

int main() {
    // PARTE 1: Binário -> Decimal
    int n, m;
    if (scanf("%d %d", &n, &m) != 2) return 0;

    long long val1 = 0;
    int bit;
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &bit) == 1) {
            val1 = (val1 << 1) | bit;
        }
    }

    long long val2 = 0;
    for (int i = 0; i < m; i++) {
        if (scanf("%d", &bit) == 1) {
            val2 = (val2 << 1) | bit;
        }
    }

    // Operações bitwise: OR, AND, XOR (Resultados em Decimal)
    printf("%lld\n", val1 | val2);
    printf("%lld\n", val1 & val2);
    printf("%lld\n", val1 ^ val2);

    // Linha em branco entre as partes
    printf("\n");

    // PARTE 2: Decimal -> Binário
    long long a, b;
    if (scanf("%lld %lld", &a, &b) != 2) return 0;

    long long resultados[3];
    resultados[0] = a | b;
    resultados[1] = a & b;
    resultados[2] = a ^ b;

    for (int r = 0; r < 3; r++) {
        long long n_atual = resultados[r];
        if (n_atual == 0) {
            printf("0\n");
        } else {
            int started = 0;
            // Percorre os bits do mais significativo para o menos significativo
            for (int i = 62; i >= 0; i--) {
                int bit_atual = (n_atual >> i) & 1;
                if (bit_atual) {
                    started = 1;
                }
                if (started) {
                    printf("%d", bit_atual);
                }
            }
            printf("\n");
        }
    }

    return 0;
}
