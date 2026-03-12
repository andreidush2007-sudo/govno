#include <stdio.h>
#include <stdlib.h>

typedef struct el
{
    int data;
    struct el* next;
    struct el* prev;
} elem;

typedef struct 
{
    elem* head, *tail;
    size_t size;
} list;

int push_back(list* lst, int val)
{
    elem* el = (elem*)malloc(sizeof(elem));
    if (el == NULL) {printf("Memmory error\n"); return -1;}
    el ->  data = val;

    if (lst -> head == NULL)
    {
        el -> next = el -> prev = el;
        lst -> head = el;
    }

    if (lst -> tail != NULL) 
    {
        el -> next = lst -> head; 
        lst -> head -> prev = el;
        el -> prev = lst -> tail; 
        lst -> tail -> next = el;
    }

    lst -> tail = el;
    lst -> size++;
}

int pop_front(list* lst)
{
    if (lst -> head == NULL) return -1;
    int val = lst -> head -> data;

    if (lst -> head == lst -> tail)
    {
        lst -> head = lst -> tail = NULL;
        lst -> size = 0;
        return val;   
    }

    elem* el = lst -> head;
    lst -> head = el -> next;
    lst -> head -> prev = lst -> tail;
    lst -> tail -> next = lst -> head;
    free(el);
    lst -> size--;
    return val;
}

void clear(list* lst)
{
    if (lst -> head == NULL) return;
    elem* cur = lst -> head;
    while (cur != lst -> tail)
    {
        int val = pop_front(lst);
        cur = lst -> head; 
    }

    int val = pop_front(lst);
    free(cur);
}

void show(list* lst)
{
    if (lst -> size == 0) return;
    elem* cur = lst -> head;
    for (int i = 0; i < lst -> size; i++)
    {
        printf("Current elemenet: %d\n", cur -> data);
        cur = cur -> next;
    }
}

int check_in(list* lst, int val)
{
    elem* head = lst -> head;
    int flag = 0;
    for (int i = 0; i < lst -> size; i++)
    {
        if (head -> data == val) {flag = 1; break;}
        head = head -> next;
    }

    return flag;
}

void add(list* l1, list* l2, list* l3)
{
    elem* h1 = l1 -> head, *h2 = l2 -> head;

    for (int i = 0; i < l1 -> size; i++) {push_back(l3, h1 -> data); h1 = h1 -> next;}

    for (int i = 0; i < l2 -> size; i++)
    {
        if (!check_in(l1, h2 -> data)) push_back(l3, h2 -> data);
        h2 = h2 -> next;
    }
}

int main()
{
    list lst1 = {.head = NULL, .size = 0, .tail = NULL};
    list lst2 = {.head = NULL, .size = 0, .tail = NULL};
    list lst3 = {.head = NULL, .size = 0, .tail = NULL};

    for (int i = 1; i < 5; i++) push_back(&lst1, i * 5);
    for (int i = 0; i < 6; i++) push_back(&lst2, i + 5);

    add(&lst1, &lst2, &lst3);
    show(&lst1);
    printf("------------\n");
    show(&lst2);
    printf("------------\n");
    show(&lst3);

    clear(&lst1); clear(&lst2); clear(&lst3);
    printf("Finish\n");

    return 0;
}