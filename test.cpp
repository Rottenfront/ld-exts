namespace std {

template <typename T> class vector {};

} // namespace std

template <typename... T> std::vector<int *> &func(T... i) {}

struct A {
    int a, c[10];
    int b;
} eax;
