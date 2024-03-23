#include "Rusty.h"
#include "Types.h"

void kPrintString(int iX, int iY, const char *pcString);

extern int add(int a, int b);

void Main() {
  kPrintString(0, 3, "C Language Kernel Started!");

  kInitializeKernel64Area();
  kPrintString(0, 4, "IA-32e Kernel Area Initialization Complete");

  while (1)
    ;
}

void kPrintString(int iX, int iY, const char *pcString) {
  CHARACTER *pstScreen = (CHARACTER *)0xB8000;
  int i;

  pstScreen += (iY * 80) + iX;
  for (i = 0; pcString[i] != 0; i++) {
    pstScreen[i].bCharactor = pcString[i];
  }
}
