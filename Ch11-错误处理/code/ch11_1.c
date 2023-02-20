#include<stdio.h>

int main(){
    FILE* fp = fopen("ch11_1.txt", "r");
    if (fp != NULL){
        char buffer[10];
        fread(buffer, 1, 5, fp);
    }
    fclose(fp);
}
