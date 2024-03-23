#include "Types.h"

void kPrintString(int iX, int iY, const char *pcString);
BOOL kInitializeKernel64Area(void);

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

BOOL kInitializeKernel64Area(void) {
  DWORD *pdwCurrentAddress;

  pdwCurrentAddress = (DWORD *)0x100000;
  while (pdwCurrentAddress < (DWORD *)0x600000) {
    *pdwCurrentAddress = 0x00;

    if (*pdwCurrentAddress != 0) {
      return FALSE;
    }
    pdwCurrentAddress++;
  }

  return TRUE;
}
