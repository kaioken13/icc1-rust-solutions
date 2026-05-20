#include <stdio.h>

int main() {
    int N;
    scanf("%d", &N);
    
    int arr[N];
    for (int i = 0; i < N; i++) {
        scanf("%d", &arr[i]);
    }
    
    int Q;
    scanf("%d", &Q);
    
    int *ptr = arr;
    
    for (int i = 0; i < Q; i++) {
        char command;
        scanf(" %c", &command);
        
        if (command == 'E') {
            int X;
            scanf("%d", &X);
            ptr = ptr + X;
        }
        else if (command == 'B') {
            int Y;
            scanf("%d", &Y);
            ptr = (int*)((char*)ptr + Y);
        }
        else if (command == 'P') {
            printf("%d\n", *ptr);
        }
        else if (command == 'D') {
            int distance = (char*)ptr - (char*)arr;
            printf("%d\n", distance);
        }
    }
    
    return 0;
}