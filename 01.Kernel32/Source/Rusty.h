#ifndef RUSTY_H
#define RUSTY_H

#include "Types.h"

void kPrintString(int iX, int iY, const char *pcString);
BOOL kInitializeKernel64Area();
BOOL kIsMemoryEnough();
void k_initialize_page_tables();
void print_cpu_manufacturer();
BOOL is_support_64();

#endif // !RUSTY_H
