#include <stdio.h>

void _isBasement(int floor, int i) {
  if (floor == -1) {
    printf("Entered Basement at: %d\n", i);
  }
}

int main() {
  char c;
  int floor = 0;
  int i = 0;
  FILE *input_file;
  input_file = fopen("input.txt", "r");

  if (input_file == NULL) {
    printf("Unable to find file.");
  }

  while ((c = fgetc(input_file)) != EOF) {
    i++;
    if (c == '(') {
      floor++;
    } else if (c == ')') {
      floor--;
    } else {
      continue;
    }
    _isBasement(floor, i);
  }

  fclose(input_file);
  printf("Floor Level: %d\n", floor);

  return 0;
}
