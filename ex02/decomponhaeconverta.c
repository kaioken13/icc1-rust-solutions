#include <stdio.h>

int main() {
    unsigned long long T;
    char S;

    if (scanf("%llu %c", &T, &S) != 2) {
        unsigned long long micro_por_segundo = 1000000;
        unsigned long long segundos_por_hora = 3600;
        unsigned long long horas_por_dia = 24;
        unsigned long long dias_por_semana = 7;

        unsigned long long segundos_totais = T / micro_por_segundo;
        unsigned long long horas_totais = segundos_totais / segundos_por_hora;

        unsigned long long semanas = horas_totais / (horas_por_dia * dias_por_semana);
        unsigned long long resto_horas_semanas = horas_totais % (horas_por_dia * dias_por_semana);

        unsigned long long dias = resto_horas_semanas / horas_por_dia;
        unsigned long long horas = resto_horas_semanas % horas_por_dia;

        printf("Decomposição do setor %c\n", S);
        printf("%llu Semana(s)\n", semanas);
        printf("%llu Dia(s)\n", dias);
        printf("%llu Hora(s)\n", horas);

        return 0;

    }
}