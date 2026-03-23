  #include <termios.h>
  #include <unistd.h>
  #include <stdint.h>
  #include <stdio.h>
  #include <stdlib.h>
  #include <time.h>

  struct termios original;

  void enable_raw_mode() {
      tcgetattr(STDIN_FILENO, &original); // save original settings
      struct termios raw = original;
      raw.c_lflag &= ~(ECHO | ICANON);   // disable echo and line buffering
      raw.c_cc[VMIN] = 0;                // non-blocking read
      raw.c_cc[VTIME] = 0;               // no timeout
      tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
  }

  void disable_raw_mode() {
      tcsetattr(STDIN_FILENO, TCSAFLUSH, &original); // restore on exit
  }

  #define WIDTH 20
  #define HEIGHT 20
  #define MIN(a, b) ((a) < (b) ? (a) : (b))

  typedef struct {
    uint8_t X;
    uint8_t Y;
  } Position;

  typedef enum {
    UP,
    DOWN,
    LEFT,
    RIGHT
  } Direction;

  struct SnakeNode {
    struct SnakeNode *next;
    Position pos;
  };

  typedef struct {
    struct SnakeNode *head;
    struct SnakeNode *tail;
    Direction dir;
  } Snake;

  uint16_t score = 0;
  void render(Snake snake, Position food) {
    char grid[HEIGHT][WIDTH];
    
    for (int i = 0; i < HEIGHT; i++) {
      for (int j = 0; j < WIDTH; j++) {
        grid[i][j] = ' ';
      }
    }


    grid[food.Y][food.X] = 'X';

    struct SnakeNode *current = snake.head;

    while (current != NULL) {
      if (current == snake.head) {
        switch(snake.dir) {
          case UP: grid[current->pos.Y][current->pos.X] = 'V'; break;
          case LEFT:  grid[current->pos.Y][current->pos.X] = '>'; break;
          case RIGHT:  grid[current->pos.Y][current->pos.X] = '<'; break;
          case DOWN:  grid[current->pos.Y][current->pos.X] = '^'; break;
        }
      } else if (current == snake.tail && current != snake.head) {
        grid[current->pos.Y][current->pos.X] = '*';
      } else {
        grid[current->pos.Y][current->pos.X] = '#';
      }
      current = current->next;
    }

    printf("\033[2J\033[H");

    // PRINT THE ARENA
    printf("+");
    for (int i = 0; i < WIDTH * 2; i++) printf("-");
    printf("+\n");
    for (int i = 0; i < HEIGHT; i++) {
      printf("|");
      for (int j = 0; j < WIDTH; j++) {
        printf("%c ", grid[i][j]);
      }
      printf("|\n");
    }
    printf("+");
    for (int i = 0; i < WIDTH * 2; i++) printf("-");
    printf("+\n");
    printf("Score: %d", score);
    printf("\n");
    printf("Speed: %d", MIN((score * 2), 125));
    printf("\n");
  }

  int snake_occupies(Snake *snake, Position pos) {
    int collision = 0;
    struct SnakeNode *current = snake->head;
    while (current != NULL) {
      if (current->pos.X == pos.X && current->pos.Y == pos.Y) {
        collision = 1;
        break;
      }
      current = current->next;
    }
    return collision;
  }


  Position spawn_food(Snake *snake) {
    Position food;
    do {
      food = (Position){ rand() % WIDTH, rand() % HEIGHT };
    } while (snake_occupies(snake, food));

    return food;
  }

  uint8_t move_snake(Snake *snake, Position *food) {
    Position new_pos;
    switch(snake->dir) {
      case UP: new_pos = (Position){
        snake->head->pos.X,
        snake->head->pos.Y == 0 ? HEIGHT - 1 : snake->head->pos.Y - 1
      };
      break;
      case DOWN: new_pos = (Position){ snake->head->pos.X, (snake->head->pos.Y + 1) % HEIGHT }; break;
      case LEFT: new_pos = (Position){
        snake->head->pos.X == 0 ? WIDTH - 1 : snake->head->pos.X - 1,
        snake->head->pos.Y
      };
      break;
      case RIGHT: new_pos = (Position) { (snake->head->pos.X + 1) % WIDTH, snake->head->pos.Y }; break;
    }

    if (snake_occupies(snake, new_pos)) {
      return 1;
    }

    struct SnakeNode *node = malloc(sizeof(struct SnakeNode));
    node->pos = new_pos;
    node->next = snake->head;
    snake->head = node;
    
    if (new_pos.X != food->X || new_pos.Y != food->Y) { 
      struct SnakeNode *current = snake->head;
      while (current->next != snake->tail) {
        current = current->next;
      }
      free(snake->tail);
      snake->tail = current;
      current->next = NULL;
    } else {
      score += 1;
      *food = spawn_food(snake);
    }
    
    return 0;
  }


  int main() {
    enable_raw_mode();
    int running = 1;

    srand(time(NULL));

    struct SnakeNode *node = malloc(sizeof(struct SnakeNode));
    node->pos = (Position){10, 10};
    node->next = NULL;

    Snake snake = {
      .head = node,
      .tail = node,
      .dir = RIGHT,
    };

    Position initial_food = spawn_food(&snake);

    while (running) {
      char key;
      int n = read(STDIN_FILENO, &key, 1);
      if (n) {
        switch(key) {
          case 'q': running = 0; break;
          case 'w': {
            if (snake.dir != DOWN) {
              snake.dir = UP;
            }
            break;
          }
          case 'a': {
            if (snake.dir != RIGHT) {
              snake.dir = LEFT;
            }
            break;

          }
          case 's': {
            if (snake.dir != UP) {
              snake.dir = DOWN;
            }
            break;

          }
          case 'd': {
            if (snake.dir !=LEFT) {
              snake.dir = RIGHT;
            }
            break;
          }
        }
      }
      uint8_t res = move_snake(&snake, &initial_food);
      if (res == 1) {
        running = 0;
      }
      render(snake, initial_food);
      usleep((200 - MIN(score * 4, 125)) * 1000);
    }
    struct SnakeNode *current = snake.head;
    // FREE SNAKE NODE MEMORY
    while (current != NULL) {
      struct SnakeNode *next = current->next;
      free(current);
      current = next;
    }
    disable_raw_mode();

    return 0;
  }
