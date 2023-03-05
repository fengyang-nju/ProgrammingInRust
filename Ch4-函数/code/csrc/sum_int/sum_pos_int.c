#include <stdarg.h>

// 计算一系列正数的值。参数列表以非正数结束。
int sum_pos_int(int x1, ...) {
    va_list ap;
    va_start(ap, x1);

    int sum = x1;
    int tmp = 0;
    while ((tmp = va_arg(ap, int)) > 0)
        sum += tmp;

    va_end(ap);

    return sum;
}