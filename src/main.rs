mod lexer;
#[cfg(test)]
mod tests;

use lexer::Lexer;

fn main() {
    let input = r#"
      /*
       * Complex C-like test script for lexer testing
       * Includes: keywords, operators, literals, comments, preprocessor directives
       */

      #define MAX_ITERATIONS 100
      #define DEBUG_MODE 1
      #define PI 3.141592653589793

      #include <stdio.h>
      #include <stdlib.h>
      #include "custom_header.h"

      // Function prototypes
      int fibonacci(int n);
      double calculate_circle_area(double radius);
      void process_data(int* data, size_t len);

      typedef struct {
          int id;
          char name[50];
          float scores[10];
          _Bool is_active;
      } Student;

      enum Color { RED = 1, GREEN, BLUE, YELLOW = 10 };

      int main(int argc, char** argv) {
          // Complex variable declarations
          volatile int x = 10, y = 0x1F, z = 075;
          static double d = 1.23e-4;
          const char* message = "Hello,\nWorld!\t";
          char raw_str[] = R"(Raw \string \with \backslashes)";
          unsigned long long big_num = 18446744073709551615ULL;

          // Complex expressions
          int result = (x << 2) | (y & 0xFF) ^ ~z;
          d += (x > 5) ? PI : -PI;

          // Control structures
          for (int i = 0; i < MAX_ITERATIONS; ++i) {
              if (i % 15 == 0) {
                  continue;
              } else if (i % 3 == 0 && DEBUG_MODE) {
                  printf("Fizz\n");
              } else if (i % 5 == 0) {
                  printf("Buzz\n");
              } else {
                  printf("%d\n", i);
              }

              switch (i % 4) {
                  case 0: x++; break;
                  case 1: y--; break;
                  case 2: x *= 2; break;
                  default: y /= 2;
              }
          }

          // Complex function calls
          int fib = fibonacci(10);
          double area = calculate_circle_area(2.5);

          // Array and pointer operations
          int matrix[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
          int* ptr = &matrix[0][0];
          ptr += 4;

          // Structure operations
          Student s = {
              .id = 1001,
              .name = "Test Student",
              .scores = {8.5, 9.0, 7.5},
              .is_active = 1
          };

          // Bit manipulation
          unsigned flags = 0;
          flags |= 0x01;
          flags &= ~0x02;

          // Complex string operations
          char* dynamic_str = malloc(100 * sizeof(char));
          sprintf(dynamic_str, "Value: %d, Area: %.2f", fib, area);

          // Memory operations
          int* data = calloc(100, sizeof(int));
          process_data(data, 100);

          // Cleanup
          free(dynamic_str);
          free(data);

          return EXIT_SUCCESS;
      }

      int fibonacci(int n) {
          if (n <= 1) {
              return n;
          }
          return fibonacci(n-1) + fibonacci(n-2);
      }

      double calculate_circle_area(double radius) {
          return PI * radius * radius;
      }

      void process_data(int* data, size_t len) {
          for (size_t i = 0; i < len; i++) {
              data[i] = (i % 2) ? i * 2 : i / 2;
          }

          #ifdef DEBUG_MODE
          printf("Data processing complete\n");
          #endif
      }
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("Input Program:\n{}\n", input);
    println!("Tokens:");
    for token in tokens {
        println!(
            "Line {:<3} Col {:<3} | {:<12} | {:?}",
            token.line, token.column, token.lexeme, token.token
        );
    }
}
