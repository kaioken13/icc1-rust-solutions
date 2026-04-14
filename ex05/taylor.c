#include <stdio.h>

int main() {
    double x;
    int n;

    scanf("%lf", &x);
    scanf("%d", &n);

    double soma = 0.0;
    double x_potencia = 1.0;
    double fatorial = 1.0;

    for (int i = 0; i < n; i++) {
        if (i == 0) {
            soma = x;
        } else {
            // Calculando a potência de dois em dois
            x_potencia *= (x * x);
            
            // Calculando fatorial a partir da base anterior
            int base_anterior = 2 * (i - 1) + 1;
            fatorial *= (base_anterior + 1) * (base_anterior + 2);
            
            double termo = x_potencia / fatorial;
            
            // Alternando o sinal
            if (i % 2 == 1) {
                soma -= termo;
            } else {
                soma += termo;
            }
        }
    }

    printf("%.10lf\n", soma);

    return 0;
}