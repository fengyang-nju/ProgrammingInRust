int main() {
    int n = 123;
    int &r = *(int *)0x12345;
    r = n; // Segmentation fault!
    return 0;
}