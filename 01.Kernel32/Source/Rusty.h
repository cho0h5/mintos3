#ifndef RUSTY_H
#define RUSTY_H

#include "Types.h"

void kPrintString(int iX, int iY, const char *pcString);
BOOL kInitializeKernel64Area();
BOOL kIsMemoryEnough();

#endif // !RUSTY_H
