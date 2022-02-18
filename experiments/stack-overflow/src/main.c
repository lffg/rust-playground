#include <stdio.h>
#include <stdint.h>

void rec(uint64_t val) {
    uint64_t *addr = &val;
    printf("%p (is %llu)\n", addr, val);
    return rec(val + 1);
}

int main() {
    rec(0);
    return 0;
}