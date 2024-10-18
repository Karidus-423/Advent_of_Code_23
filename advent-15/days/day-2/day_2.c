#include <stdio.h>
#include <stdlib.h>

int min(int a, int b, int c) {
    int smallest = 0;

    if (a <= b && a <= c) {
        smallest = a;
    } else if (b <= a && b <= c) {
        smallest = b;
    } else if (c <= a && c <= a) {
        smallest = c;
    }
    return smallest;
}

int bow(int a, int b, int c) {
    int smallest = 0;
    int small = 0;

    if (a <= b && a <= c) {
        smallest = a;

        if (b <= c) {
            small = b;
        } else {
            small = c;
        }
    } else if (b <= a && b <= c) {
        smallest = b;

        if (a <= c) {
            small = a;
        } else {
            small = c;
        }
    } else if (c <= a && c <= b) {
        smallest = c;

        if (a <= b) {
            small = a;
        } else {
            small = b;
        }
    }

    // Part1
    // return smallest;
    int wrap = (2 * smallest) + (2 * small);
    int bow = a * b * c;
    int total_length = wrap + bow;
    return total_length;
}

int main() {
    FILE *file_ptr = fopen("./input.txt", "r");

    if (file_ptr == NULL) {
        printf("Unable to find file.");
        exit(0);
    }

    int l = 0;
    int h = 0;
    int w = 0;
    int total = 0;
    int small = 0;
    int feet = 0;
    int ribbon = 0;
    int ribbon_len = 0;

    while (fscanf(file_ptr, "%dx%dx%d", &l, &w, &h) != EOF) {
        int s1 = (l * w);
        int s2 = (w * h);
        int s3 = (h * l);
        feet = (2 * s1) + (2 * s2) + (2 * s3);

        small = min(s1, s2, s3);
        ribbon = bow(l, w, h);

        total = total + feet + small;
        ribbon_len = ribbon_len + ribbon;
    }
    printf("Total Square Feet %d\n", total);
    printf("Total Ribbon Feet %d\n", ribbon_len);
    fclose(file_ptr);
}
