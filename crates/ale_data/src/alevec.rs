use std::marker::PhantomData;

pub struct AleVec<T> {
    generation: usize,
    vec: Vec<Data<T>>,
    len: usize,
}

// wrapper around the actual data
pub struct Data<T> {
    object: T,
    is_deleted: bool,
}

// key that will be generated by AleVec and be given to the caller
// will identify an entry in the vec
pub struct Key<T> {
    generation: usize,
    index: usize,
    phantom: PhantomData<T>,
}

impl<T> AleVec<T> {
    pub fn new() -> AleVec<T> {
        AleVec {
            generation: 0,
            vec: vec![],
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> AleVec<T> {
        AleVec {
            generation: 0,
            vec: Vec::with_capacity(capacity),
            len: 0,
        }
    }

    pub fn push(&mut self, t: T) -> Key<T> {
        self.vec.push(Data {
            object: t,
            is_deleted: false,
        });
        self.len += 1;
        Key {
            generation: self.generation,
            index: self.vec.len() - 1,
            phantom: Default::default(),
        }
    }

    pub fn remove(&mut self, key: Key<T>) {
        if key.generation != self.generation {
            return;
        }
        self.vec[key.index].is_deleted = true;
        self.len -= 1;
    }

    pub fn clear(&mut self) {
        self.vec.clear();
        self.generation += 1;
        self.len = 0;
    }

    pub fn reallocate(&mut self) {
        let mut new_vec = Vec::with_capacity(self.len);
        for item in self.vec.drain(..) {
            if !item.is_deleted {
                new_vec.push(item)
            }
        }
        self.vec = new_vec;
    }

    pub fn get(&self, key: Key<T>) -> Option<&T> {
        if key.generation != self.generation {
            return None;
        }
        return match self.vec.get(key.index) {
            None => None,
            Some(d) => {
                if d.is_deleted {
                    return None;
                }
                Some(&d.object)
            }
        };
    }

    pub fn get_mut(&mut self, key: Key<T>) -> Option<&mut T> {
        if key.generation != self.generation {
            return None;
        }
        return match self.vec.get_mut(key.index) {
            None => None,
            Some(d) => {
                if d.is_deleted {
                    return None;
                }
                Some(&mut d.object)
            }
        };
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> AleVecIter<T> {
        return AleVecIter {
            alevec: self,
            index: 0,
        };
    }

    pub fn iter_mut(&mut self) -> AleVecIterMut<T> {
        return AleVecIterMut {
            alevec: self,
            index: 0,
        };
    }
}

pub struct AleVecIter<'a, T> {
    alevec: &'a AleVec<T>,
    index: usize,
}

impl<'a, T> Iterator for AleVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.index..self.alevec.vec.len() {
            let d = &self.alevec.vec[i];
            self.index += 1;
            if !d.is_deleted {
                return Some(&d.object);
            }
        }
        None
    }
}

pub struct AleVecIterMut<'a, T> {
    alevec: &'a mut AleVec<T>,
    index: usize,
}

impl<'a, T> Iterator for AleVecIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.index..self.alevec.vec.len() {
            unsafe {
                let d = self.alevec.vec.as_mut_ptr().offset(i as isize);
                self.index += 1;
                if !(*d).is_deleted {
                    return Some(&mut (*d).object)
                }
            }
        }
        None
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Key {
            generation: self.generation,
            index: self.index,
            phantom: Default::default()
        }
    }
}

impl <T> Copy for Key<T> {}

#[test]
fn test_alevec() {
    let mut v = AleVec::new();
    let k1 = v.push(10);
    let k2 = v.push(20);
    let k3 = v.push(30);

    v.remove(k2);
    assert_eq!(v.get(k2), None);
    assert_eq!(v.get(k1), Some(&10));
    assert_eq!(v.len(), 2);

    v.clear();
    assert_eq!(v.len(), 0);

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(10);
    v.push(12);

    let mut result = vec![1,2,3,10,12];
    let mut ctr = 0;
    for num in v.iter(){
        assert_eq!(num, &result[ctr]);
        ctr += 1;
    }

    let mut ctr = 0;
    for num in v.iter_mut() {
        assert_eq!(num, &mut result[ctr]);
        ctr += 1;
    }

    let mut ctr = 0;
    for mut num in v.iter_mut() {
        let mut p = 20;
        num = &mut p;
        assert_eq!(num, &mut 20);
        ctr += 1;
    }

    let mut realloc_vec = AleVec::with_capacity(5);
    realloc_vec.push(1);
    let key = realloc_vec.push(2);
    realloc_vec.push(3);

    realloc_vec.remove(key);

    assert_eq!(realloc_vec.vec.len(), 3);
    realloc_vec.reallocate();
    assert_eq!(realloc_vec.vec.len(), 2);
}