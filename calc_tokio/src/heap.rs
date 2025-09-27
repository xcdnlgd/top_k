#[derive(Debug)]
pub struct MinHeap<T: PartialOrd> {
    _data: Vec<T>,
}

impl<T: PartialOrd> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { _data: Vec::new() }
    }
    pub fn get_root(&self) -> Option<&T> {
        self._data.first()
    }
    pub fn len(&self) -> usize {
        self._data.len()
    }
    pub fn is_empty(&self) -> bool {
        self._data.is_empty()
    }
    pub fn as_vec(&self) -> &Vec<T> {
        &self._data
    }
}

impl<T: PartialOrd> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialOrd> MinHeap<T> {
    fn get_parent_index(&self, index: usize) -> Option<usize> {
        match index {
            0 => None,
            other => Some((other - 1) / 2),
        }
    }
    fn get_l_child_index(&self, index: usize) -> Option<usize> {
        let potential_l_child = index * 2 + 1;
        if potential_l_child >= self._data.len() {
            None
        } else {
            Some(potential_l_child)
        }
    }
    fn get_r_child_index(&self, index: usize) -> Option<usize> {
        let potential_r_child = index * 2 + 2;
        if potential_r_child >= self._data.len() {
            None
        } else {
            Some(potential_r_child)
        }
    }
    fn bubble_up(&mut self, index: usize) {
        let mut i = index;
        while let Some(parent_index) = self.get_parent_index(i) {
            if self._data[i] >= self._data[parent_index] {
                break;
            }
            self._data.swap(i, parent_index);
            i = parent_index;
        }
    }
    fn bubble_down(&mut self, index: usize) {
        let mut i = index;
        while let Some(l_child_index) = self.get_l_child_index(i) {
            let min_child_index = if let Some(r_child_index) = self.get_r_child_index(i) {
                if self._data[l_child_index] < self._data[r_child_index] {
                    l_child_index
                } else {
                    r_child_index
                }
            } else {
                l_child_index
            };

            if self._data[i] < self._data[min_child_index] {
                break;
            }
            self._data.swap(i, min_child_index);
            i = min_child_index;
        }
    }

    pub fn insert(&mut self, element: T) {
        let index = self._data.len();
        self._data.push(element);
        self.bubble_up(index);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self._data.is_empty() {
            return None;
        }
        let last_index = self._data.len() - 1;
        self._data.swap(0, last_index);
        let root = self._data.pop();
        self.bubble_down(0);
        root
    }
}
