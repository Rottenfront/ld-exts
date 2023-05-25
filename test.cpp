template <typename T, typename size_t = unsigned long long>
class Vector {
private:
    T *arr;
    size_t size;
    size_t len;
public:
    Vector(size_t length) {
        this->len = length;
        this->size = length / 20 * 20 + 20;
        this->arr = new T[this->size];
    }

    void push_back(T val) {
        if (this->len == this->size) {
            size += 20;
            this->arr = new T[this->arr];
            this->arr[this->len] = val;
            this->len += 1;
        } else {
            this->arr[len] = val;
            this->len += 1;
        }
    }

    size_t length() const {
        return len;
    }
};
