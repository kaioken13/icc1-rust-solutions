#include <stdio.h>

int main() {
    unsigned short a, b, c;

    // Lendo os três valores short unsigned int (2 bytes cada)
    if (scanf("%hu %hu %hu", &a, &b, &c) != 3) {
        return 0;
    }

    a = a & ~1;
    b = b & ~1;
    c = c & ~1;

    unsigned long long resultado = 0;
    
    resultado = ((unsigned long long)c << 32) | ((unsigned long long)b << 16) | (unsigned long long)a;

    // Saída: O valor resultante seguido de quebra de linha
    printf("%llu\n", resultado);

    return 0;
}