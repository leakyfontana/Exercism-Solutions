#include "armstrong_numbers.h"
#include <math.h>
#include <stdbool.h>
#include <stdio.h>


bool is_armstrong_number(int candidate) {

    if (candidate == 0) {
      return true;
    }

    int temp = candidate;
    int dgt = 0;
    while ((temp % 10) > 0) {
        dgt++;
        temp = temp / 10;
    }

    printf("%d", dgt);

    int total = 0;
    int tmp2 = candidate;
    int n;
    while (tmp2 > 0) {
        n = tmp2 % 10;

        total = total + pow(n, dgt);

        tmp2 = tmp2 / 10;
    }

    if (total == candidate) {
      return true;
    }

    return false;
}
