#include <stdio.h>
#include <stdlib.h>
#include "cJSON.h"

int main() 
{
    cJSON *root = cJSON_Parse("{\"name\":\"Tom\",\"age\":18,\"hobbies\":[\"coding\",\"music\"]}");
    if (!root) {
        printf("parse error\n");
        return 1;
    }
    char *pretty2 = cJSON_PrintPretty(root, 2);
    printf("Pretty (2 spaces):\n%s\n", pretty2);
    free(pretty2);
    char *pretty4 = cJSON_PrintPretty(root, 4);
    printf("Pretty (4 spaces):\n%s\n", pretty4);
    free(pretty4);
    char *old = cJSON_Print(root);
    printf("Original formatted (tabs):\n%s\n", old);
    free(old);
    cJSON_Delete(root);
    return 0;
}