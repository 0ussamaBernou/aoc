#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char get_duplicate(char *left, char *right);
int get_priority(char dup);

int main() {
  FILE *file;
  char *line = NULL;
  size_t len = 0;
  ssize_t read;

  // Open the file for reading
  file = fopen("day3.input", "r");

  // Check if the file opened successfully
  if (file == NULL) {
    perror("Error opening the file");
    return 1;
  }

  char *left;
  char *right;
  char dup;
  int priority = 0, priorities = 0;

  // Read and process the file line by line
  while ((read = getline(&line, &len, file)) != -1) {
    // Do something with the line, e.g., print it
    size_t length = strlen(line);

    // Allocate memory for left and right
    left = (char *)malloc((length / 2) + 1);
    right = (char *)malloc((length / 2) + 1);

    if (left == NULL || right == NULL) {
      perror("Memory allocation failed");
      return 1;
    }

    // Use strncpy to copy the left and right halves
    strncpy(left, line, length / 2);
    left[length / 2] = '\0'; // Null-terminate the string

    strncpy(right, line + (length / 2), length / 2);
    right[length / 2] = '\0'; // Null-terminate the string

    dup = get_duplicate(left, right);
    priority = get_priority(dup);
    priorities += priority;
  }

  printf("priorities: %d\n", priorities);

  if (left)
    free(left);
  if (right)
    free(right);
  // Close the file when you're done
  fclose(file);

  // Free the dynamically allocated memory for the line
  if (line) {
    free(line);
  }

  return 0;
}

char get_duplicate(char *left, char *right) {

  size_t leftlen = strlen(left);
  size_t rightlen = strlen(right);
  for (size_t i = 0; i < leftlen; i++) {
    for (size_t j = 0; j < rightlen; j++) {
      if (left[i] == right[j]) {
        return left[i];
      }
    }
  }
  return '0';
}

int get_priority(char dup) {
  if (dup > 64 && dup < 91) {
    return dup - 38;
  } else if (dup > 96 && dup < 123) {
    return dup - 96;
  }
}
