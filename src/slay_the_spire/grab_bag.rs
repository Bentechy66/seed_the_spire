use crate::slay_the_spire::rng::Rng;

pub struct GrabBag<T> {
    entries: Vec<(T, f64)>,
    total_weight: f64,
}

impl<T: Copy> GrabBag<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            total_weight: 0.0,
        }
    }

    pub fn add(&mut self, element: T, weight: f64) {
        self.entries.push((element, weight));
        self.total_weight += weight;
    }

    pub fn any(&self) -> bool {
        !self.entries.is_empty()
    }

    pub fn refill_uniform(&mut self, items: &[T]) {
        self.entries.clear();
        self.total_weight = 0.0;
        for &x in items {
            self.add(x, 1.0);
        }
    }

    pub fn grab_and_remove(&mut self, rng: &mut Rng) -> Option<T> {
        let idx = self.grab_index_weighted(rng)?;
        Some(self.remove_at(idx))
    }

    pub fn grab_and_remove_if(&mut self, rng: &mut Rng, predicate: impl Fn(T) -> bool) -> Option<T> {
        if !self.entries.iter().any(|(e, _)| predicate(*e)) {
            return None;
        }
        loop {
            let idx = self.grab_index_weighted(rng)?;
            if predicate(self.entries[idx].0) {
                return Some(self.remove_at(idx));
            }
        }
    }

    fn grab_index_weighted(&self, rng: &mut Rng) -> Option<usize> {
        if self.entries.is_empty() || self.total_weight <= 0.0 {
            return None;
        }
        let roll = rng.next_double() * self.total_weight;
        let mut acc = 0.0;
        for (i, (_, w)) in self.entries.iter().enumerate() {
            acc += *w;
            if roll < acc {
                return Some(i);
            }
        }
        None
    }

    fn remove_at(&mut self, index: usize) -> T {
        let (item, w) = self.entries.remove(index);
        self.total_weight -= w;
        item
    }
}

impl<T: Copy> Default for GrabBag<T> {
    fn default() -> Self {
        Self::new()
    }
}
