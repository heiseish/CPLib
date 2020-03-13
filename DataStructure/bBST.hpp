#include <bits/stdc++.h>
using namespace std;


template <class T>
class BinarySearchTree {
private:
	template <class T>
	class Vertex{
	public:
		T key;
		Vertex<T> *left;
		Vertex<T> *right;
		Vertex<T> *parent;
		int height;
		int size;
		Vertex(T p) : key(p), parent(NULL), right(NULL), left(NULL), height(0), size(1) { }
	};
    Vertex<T>* root;
    Vertex<T>* get(Vertex<T>* v, T p) {
        if (v == NULL)  return NULL;                    
        else if (v->key == p) return v;               
        else if (v->key < p)  return get(v->right, p);    
        else return get(v->left, p);       
    }
    Vertex<T>* add(Vertex<T>* v, T p) {
        Vertex<T>* w;
        if (v == NULL)  v = new Vertex<T>(p);
        else if (v->key < p) {
            w = add(v->right, p);
            v->right = w;
            w->parent = v;
        } else  {
            w = add(v->left, p); 
            v->left = w;
            w->parent = v;
        }
        updateProperty(v);
        return checkVariant(v);
    }

    Vertex<T>* remove(Vertex<T>* v, T p) {
        Vertex<T>* w;
        if (v == NULL)  return NULL; // impossible, since remove guarantee to succeed
        else if (v->key == p) {
            Vertex<T>* par = v->parent;
            Vertex<T>* left = v->left;
            Vertex<T>* right = v->right;
            if (right == NULL && left == NULL) v = NULL;
            else if (right != NULL && left == NULL) {
                right->parent = par;
                right->left = left;
                v = right;
            } else if (right == NULL && left != NULL) {
                left->parent = par;
                left->right = right;
                v = left;
            } else {
                w = successor(v);
                v->key = w->key;
                v->right = remove(v->right, w->key);
            }
            
        } else if (v->key < p) {
            w = remove(v->right, p);
            v->right = w;
            if (w != NULL) w->parent = v;
        } else  {
            w = remove(v->left, p); 
            v->left = w;
            if (w != NULL) w->parent = v;
        }
        updateProperty(v);
        return checkVariant(v);

    }

    Vertex<T>* rotateLeft(Vertex<T>* v) {
        Vertex<T>* r = v->right;
        r->parent = v->parent;
        v->parent = r;
        v->right = r->left;
        if (r->left != NULL) r->left->parent = v;
        r->left = v;
        updateProperty(v);
        updateProperty(r);
        return r;
    }

    Vertex<T>* rotateRight(Vertex<T>* v) {
        Vertex<T>* l = v->left;
        l->parent = v->parent;
        v->parent = l;
        v->left = l->right;
        if (l->right != NULL) l->right->parent = v;
        l->right = v;
        updateProperty(v);
        updateProperty(l);
        return l;
    }

    int balanceFactor(Vertex<T>* v) {
        if (v == NULL) return 0;
        int leftHeight = v->left == NULL ? -1 : v->left->height;
        int rightHeight = v->right == NULL ? -1 : v->right->height;
        return leftHeight - rightHeight;
    }

    Vertex<T>* checkVariant(Vertex<T>* v) {
        if (v == NULL) return NULL;
        Vertex<T>* w;
        if (balanceFactor(v) == 2 && 0 <= balanceFactor(v->left) && balanceFactor(v->left) <= 1) {
            v = rotateRight(v);
        } else if (balanceFactor(v) == 2 && balanceFactor(v->left) == -1) {
            w = rotateLeft(v->left);
            w->parent = v;
            v->left = w;
            v = rotateRight(v);
        } else if (balanceFactor(v) == -2 && -1 <= balanceFactor(v->right) && balanceFactor(v->right) <= 0) {
            v = rotateLeft(v);
        } else if (balanceFactor(v) == -2 && balanceFactor(v->right) == 1) {
            w = rotateRight(v->right);
            w->parent = v;
            v->right = w;
            v = rotateLeft(v);
        }
        return v;
    }

    void updateProperty(Vertex<T>* v) {
        if (v == NULL) return;
        int leftSize = v->left == NULL ? 0 : v->left->size;
        int rightSize = v->right == NULL ? 0 : v->right->size;
        v->size = leftSize + rightSize + 1;

        int leftHeight = v->left == NULL ? -1 : v->left->height;
        int rightHeight = v->right == NULL ? -1 : v->right->height;
        v->height = max(leftHeight, rightHeight) + 1;
    }

    

    int rank(Vertex<T>* v, T p) {
        if (v == NULL) return 0;
        if (v->key == p) {
            if (v->left == NULL) return 0;
            return v->left->size;
        } else if (v->key < p) {
            int left = v->left == NULL ? 0 : v->left->size;
            return 1 + left + rank(v->right, p);
        } else return rank(v->left, p);
    }

    Vertex<T>* findNearestVertexToTheRight(Vertex<T>* v, T p) {
        if (v == NULL) return NULL;
        if (v->key == p) return v;               
        else if (v->key < p && v->right != NULL)  return findNearestVertexToTheRight(v->right, p);   
        else if (v->key < p && v->right == NULL)  return successor(v);   
        else if (v->key > p && v->left != NULL)  return findNearestVertexToTheRight(v->left, p);
        else if (v->key > p && v->left == NULL)  return v;   
        return NULL;
    }

    Vertex<T>* findNearestVertexToTheLeft(Vertex<T>* v, T p) {
        if (v == NULL) return NULL;
        if (v->key == p) return predecessor(v);               
        else if (v->key < p && v->right != NULL)  return findNearestVertexToTheLeft(v->right, p);   
        else if (v->key < p && v->right == NULL)  return v;   
        else if (v->key > p && v->left != NULL)  return findNearestVertexToTheLeft(v->left, p);
        else if (v->key > p && v->left == NULL)  return predecessor(v);   
        return NULL;
    }

    Vertex<T>* successor(Vertex<T>* v) {
        if (v->right != NULL)           
            return findMin(v->right);  
        else {
            Vertex<T>* par = v->parent;
            Vertex<T>* cur = v;
            // if par(ent) is not root and cur(rent) is its right children
            while ((par != NULL) && (cur == par->right)) {
                cur = par;                                         // continue moving up
                par = cur->parent;
            }
            return par == NULL ? NULL : par;           // this is the successor of T
        }
    }

    Vertex<T>* findMin(Vertex<T>* v) {
        if (v == NULL) return NULL;
        else if (v->left == NULL) return v;                    // this is the min
        else return findMin(v->left);           // go to the left
    }

    Vertex<T>* predecessor(Vertex<T>* v) {
        if (v->left != NULL)                       // this subtree has right subtree
            return findMax(v->left);  // the successor is the minimum of right subtree
        else {
            Vertex<T>* par = v->parent;
            Vertex<T>* cur = v;
            // if par(ent) is not root and cur(rent) is its right children
            while ((par != NULL) && (cur == par->left)) {
                cur = par;                                         // continue moving up
                par = cur->parent;
            }
            return par == NULL ? NULL : par;           // this is the successor of T
        }
    }
    Vertex<T>* findMax(Vertex<T>* v) {
        if (v == NULL) return NULL;
        else if (v->right == NULL) return v;                    // this is the min
        else return findMax(v->right);           // go to the left
    }

public:
    BinarySearchTree(): root(NULL) { }

    T get(T p) {
        Vertex<T>* v = get(root, p);
        return v == NULL ? NULL : v->key;
    }

    void add(T p) {
        if (root == NULL) root = new Vertex<T>(p);
        else root = add(root, p);
    }

    void remove(T p) {
        if (root == NULL) return;
        else root = remove(root, p);
    }

    int rank(T p) {
        return rank(root, p);
 
    }

    int getSize() {
        return root->size;
    }

    int subSet(T START, T END) {
        Vertex<T>* s = findNearestVertexToTheRight(root, START);
        Vertex<T>* e = findNearestVertexToTheLeft(root, END);
        if (s == NULL || e == NULL) return 0;
        else {
            int start = rank(s->key);
            int end = rank(e->key);
            return end - start + 1;

        }

    }
};