(define _length (lambda (items sofar)
    (cond
        ((eq? items '())
            sofar
        )
        (else 
            (_length (cdr items) (+ sofar 1))
        )
    )
))
(define length (lambda (items)
    (_length items 0)
))

(length '())
(length '(1))
(length '(1 2))
(length '(1 2 3))

(define split_left (lambda (items index)
    (cond
        ((< index 1) '())
        (else
            (cons
                (car items)
                (split_left (cdr items) (- index 1))
            )
        )
    )
))
(split_left '(1 2 3) 0)
(split_left '(1 2 3) 1)
(split_left '(1 2 3) 2)
(split_left '(1 2 3) 3)

; comments!

(define split_right (lambda (items index)
    (cond
        ((< index 1) items)
        (else
            (split_right (cdr items) (- index 1))
        )
    )
))
(split_right '(1 2 3) 0)
(split_right '(1 2 3) 1)
(split_right '(1 2 3) 2)
(split_right '(1 2 3) 3)

(define combine (lambda (left right)
    (cond
        ((eq? left '()) right)
        ((eq? right '()) left)
        ((< (car left) (car right))
            (cons (car left) (combine (cdr left) right))
        )
        (else
            (cons (car right) (combine left (cdr right)))
        )
    )
))

(combine '(1 2) '(3 4))
(combine '(1 2 3) '(4))
(combine '(2 4) '(1 3))

(define mergesort (lambda (items)
    (cond
        ((<= (length items) 1)
            items
        )
        (else
            (combine
                (mergesort (split_left items (/ (length items) 2)))
                (mergesort (split_right items (/ (length items) 2)))
            )
        )
    )
))

(mergesort '(1 2 3 4))
(mergesort '(1 4 2 3))
(mergesort '(4 3 2 1))
(mergesort '(9 4 1 8 1 2 0 4 3 2 4 8 1 9 2 3 5 2 8 7 9 0 2 2 3 4))