struct Compass {
    unsigned int vert;
    unsigned int col;
    unsigned int ind;
    unsigned int vao;
};

typedef struct Compass Compass;

void make_compass(Compass* compass);
void draw_compass(Compass* compass, int col_loc);
