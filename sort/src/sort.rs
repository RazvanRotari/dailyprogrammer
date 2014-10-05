
pub mod sort {
    pub fn qsort<T: Ord>(xs:&mut [T], step:|i: uint, j: uint|) {
        sort(xs, 0, step)
    }

    fn sort<T: Ord>(xs:&mut [T], start:uint, step:|i: uint, j: uint|) {
        if xs.len() <=1 {
            return
        }
        let p = pivot(xs);   

        let p = partition(xs, p, start, |i, j| step(i, j));

        sort(xs.slice_to_mut(p), start, |i, j| step(i, j));
        sort(xs.slice_from_mut(p+1), start+p+1, |i, j| step(i, j));
    }
    fn partition<T: Ord>(xs:&mut [T], p:uint, start:uint, step:|i: uint, j: uint|) -> uint {
        if xs.len() <= 1 {
            return p;
        }
        let lasti = xs.len() - 1;
        let mut nextp = 0u;
        xs.swap(p, lasti);
        step(start+p, start+lasti);
        for i in range(0u, lasti) {
            if xs[i] <= xs[lasti] {
                xs.swap(i, nextp);
                step(start+i, start+nextp);
                nextp += 1;
            }
        }
        xs.swap(lasti, nextp);
        step(start+lasti, start+nextp);
        nextp
    }

    fn pivot<T: Ord>(xs:&[T]) -> uint{
        let (l, r) = (0, xs.len() - 1);
        let m = r / 2; 
        let (left, middle, right) = (&xs[l], &xs[m], &xs[r]);
        if middle >= left && middle <= right {
            m
        } else if left >= middle && left <= right {
            l
        } else {
            r
        }
    }

    #[cfg(test)]
    #[test]
    fn test_sort() {
        let mut origin = [32i32,421,521,33,51,63251,34,1,2,4,0];
        let mut finish = [0, 1, 2, 4, 32, 33, 34, 51, 421, 521, 63251];
        qsort(origin, |_:uint, _:uint| {});
        //let compare = origin.as_slice().cmp(&finish.as_slice());
        let compare = origin.cmp(&finish.as_slice());
        println!("Origin: {}", origin.as_slice());
        match compare {
            Equal => return,
            _ => fail!("Sort failed"),
        }
    }

    #[cfg(test)]
    #[test]
    fn test_sorted() {
        let mut origin = [1i32, 2, 3];
        qsort(origin, |_:uint, _:uint| {});
        let compare = origin.cmp(&origin.as_slice());
        println!("Origin: {}", origin.as_slice());
        match compare {
            Equal => return,
            _ => fail!("Sort failed"),
        }
    }

    #[cfg(test)]
    #[test]
    fn test_pivot() {
        let origin = [1i32, 2, 3];
        assert!(pivot(origin) == 1);
    }
}

