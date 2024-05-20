#include <cassert>

int main() {
   int n = 123;
   int &r = n;
   assert(r == 123);
   r = 321;
}