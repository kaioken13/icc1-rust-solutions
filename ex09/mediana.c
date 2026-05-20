#include <stdio.h>

int main() {
    int k;
    
    scanf("%d", &k);
    
    int arr[k];
    
    for (int i = 0; i < k; i++) {
        scanf("%d", &arr[i]);
    }
    
    for (int i = 0; i < k - 1; i++) {
        for (int j = 0; j < k - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    
    float mediana;
    if (k % 2 == 1) {
        mediana = arr[k / 2];
    } else {
        mediana = (arr[k / 2 - 1] + arr[k / 2]) / 2.0;
    }
    
    printf("%.1f\n", mediana);
    
    return 0;
}