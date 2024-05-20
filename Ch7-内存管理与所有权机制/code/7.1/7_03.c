int main() {
    char *a = malloc(512);
    char *b = malloc(256);
    char *c;
    strcpy(a, "A here");
    free(a);
    c = malloc(512);
    strcpy(c, "C here");
    printf("a: %p, %s\n", a, a);
    printf("c: %p, %s\n", c, c);
    return 0;
}