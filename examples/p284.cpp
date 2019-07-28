#include <vector>

using std::vector;

class Iterator {
    struct Data;
	Data* data;
public:
	Iterator(const vector<int>& nums);
	Iterator(const Iterator& iter);
	virtual ~Iterator();
	// Returns the next element in the iteration.
	int next();
	// Returns true if the iteration has more elements.
	bool hasNext() const;
};


class PeekingIterator : public Iterator {
    bool peeked;
    bool nextEnd;
    int head;
public:
	PeekingIterator(const vector<int>& nums) : Iterator(nums), peeked(false), nextEnd(false), head(0) {
	    // Initialize any member here.
	    // **DO NOT** save a copy of nums and manipulate it directly.
	    // You should only use the Iterator interface methods.
	    
	}

    // Returns the next element in the iteration without advancing the iterator.
	int peek() {
        if (!this->peeked) {
            this->head = this->next();
            this->peeked = true;
            this->nextEnd = !Iterator::hasNext();
        }
        return this->head;
	}

	// hasNext() and next() should behave the same as in the Iterator interface.
	// Override them if needed.
	int next() {
        if (this->peeked) {
            this->peeked = false;
            if (this->nextEnd) {
                this->nextEnd = false;
            }
            return this->head;
        }
        return Iterator::next();
	}

	bool hasNext() const {
        if (this->nextEnd) {
            return true;
        }
        return Iterator::hasNext();
	}
};