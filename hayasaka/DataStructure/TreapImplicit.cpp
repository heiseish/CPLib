/**
Implicit treap is a simple modification of the regular treap which is a very powerful data structure. In fact, implicit treap can be considered as an array with the following procedures implemented (all in O(logN)

in the online mode):

    Inserting an element in the array in any location
    Removal of an arbitrary element
    Finding sum, minimum / maximum element etc. on an arbitrary interval
    Addition, painting on an arbitrary interval
    Reversing elements on an arbitrary interval


- Insert element.
Suppose we need to insert an element at position pos. We divide the treap into two parts, 
which correspond to arrays [0..pos-1] and [pos..sz]; to do this we call split (T, T1, T2, pos).
 Then we can combine tree T1 with the new vertex by calling merge (T1, T1, new_item) 
 (it is easy to see that all preconditions are met). Finally, we combine trees T1 and T2 back
  into T by calling merge (T, T1, T2
).
- Delete element.
This operation is even easier: find the element to be deleted T, perform merge of its 
children L and R, and replace the element T with the result of merge. 
In fact, element deletion in the implicit treap is exactly the same as in the regular treap.
- Find sum / minimum, etc. on the interval.
First, create an additional field F in the item structure to store the value of the target 
function for this node's subtree. This field is easy to maintain similarly to maintaining 
sizes of subtrees: create a function which calculates this value for a node based on values 
for its children and add calls of this function in the end of all functions which modify the tree.
Second, we need to know how to process a query for an arbitrary interval [A; B].
To get a part of tree which corresponds to the interval [A; B], we need to call split (T, T1
, T2, A), and then split (T2, T2, T3, B - A + 1): after this T2 will consist of all the elements 
in the interval [A; B], and only of them. Therefore, the response to the query will be stored in 
the field F of the root of T2. After the query is answered, the tree has to be restored by calling 
merge (T, T1, T2) and merge (T, T, T3
).
- Addition / painting on the interval.
We act similarly to the previous paragraph, but instead of the field F we will store a field add 
which will contain the added value for the subtree (or the value to which the subtree is painted). 
Before performing any operation we have to "push" this value correctly - i.e. change T→L→add
and T→R→add
, and to clean up add in the parent node. This way after any changes to the tree the information will not be lost.
- Reverse on the interval.
This is again similar to the previous operation: we have to add boolean flag ‘rev’ and set it to 
true when the subtree of the current node has to be reversed. "Pushing" this value is a 
bit complicated - we swap children of this node and set this flag to true for them.
*/


typedef struct item * pitem;
struct item {
    int prior, value, cnt;
    bool rev;
    pitem l, r;
};

int cnt (pitem it) {
    return it ? it->cnt : 0;
}

void upd_cnt (pitem it) {
    if (it)
        it->cnt = cnt(it->l) + cnt(it->r) + 1;
}

void push (pitem it) {
    if (it && it->rev) {
        it->rev = false;
        swap (it->l, it->r);
        if (it->l)  it->l->rev ^= true;
        if (it->r)  it->r->rev ^= true;
    }
}

void merge (pitem & t, pitem l, pitem r) {
    push (l);
    push (r);
    if (!l || !r)
        t = l ? l : r;
    else if (l->prior > r->prior)
        merge (l->r, l->r, r),  t = l;
    else
        merge (r->l, l, r->l),  t = r;
    upd_cnt (t);
}

void split (pitem t, pitem & l, pitem & r, int key, int add = 0) {
    if (!t)
        return void( l = r = 0 );
    push (t);
    int cur_key = add + cnt(t->l);
    if (key <= cur_key)
        split (t->l, l, t->l, key, add),  r = t;
    else
        split (t->r, t->r, r, key, add + 1 + cnt(t->l)),  l = t;
    upd_cnt (t);
}

void reverse (pitem t, int l, int r) {
    pitem t1, t2, t3;
    split (t, t1, t2, l);
    split (t2, t2, t3, r-l+1);
    t2->rev ^= true;
    merge (t, t1, t2);
    merge (t, t, t3);
}

void output (pitem t) {
    if (!t)  return;
    push (t);
    output (t->l);
    printf ("%d ", t->value);
    output (t->r);
}

void Insert()