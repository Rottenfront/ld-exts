void *simple_memccpy(void *, const void *, int, size_t);
void *stupid_memccpy(void *, const void *, int, size_t);

void *simple_memccpy(void *dst, const void *src, int c, size_t n) {
    const char *s = src;
    char *d = dst;

    while (n-- > 0)
        if ((*d++ = *s++) == (char)c)
            return d;

    return NULL;
}

void *stupid_memccpy(void *dst, const void *src, int c, size_t n) {
    const char *s = src;
    char *d = dst;

    while (n-- > 0) {
        *d = *s;
        if (*d == (char)c)
            return d + 1;
        d++;
        s++;
    }

    return NULL;
}
