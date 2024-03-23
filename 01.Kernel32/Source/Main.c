#include "Types.h"

void kPrintString(int iX, int iY, const char *pcString);
BOOL kInitializeKernel64Area();
BOOL kIsMemoryEnough();

extern int add(int a, int b);

void Main() {
  kPrintString(0, 3, "C Language Kernel Start.....................[Pass]");
  kPrintString(0, 4, "Mininum Memory Size Check...................[    ]");
  if (kIsMemoryEnough() == FALSE) {
    kPrintString(0, 4, "Fail");
    kPrintString(0, 5, "Not Enough Memory.");
    while (1)
      ;
  }
  kPrintString(45, 4, "Pass");

  kPrintString(0, 5, "IA-32e Kernel Area Initialization...........[    ]");
  if (kInitializeKernel64Area() == FALSE) {
    kPrintString(45, 5, "Fail");
    kPrintString(0, 6, "Kernel Area Initialization Fail!");
    while (1)
      ;
  }
  kPrintString(45, 5, "Pass");

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
