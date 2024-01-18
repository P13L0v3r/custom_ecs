#[derive(Debug, Clone)]
pub(crate) struct ValidEntityRange {
    pub lower_bound: usize,
    pub upper_bound: Option<usize>,
}

impl ValidEntityRange {
    pub(crate) fn new(lower_bound: usize, upper_bound: Option<usize>) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }

    pub(crate) fn split_at(&mut self, index: &usize) -> Option<ValidEntityRange> {
        if *index >= self.lower_bound {
            let new_range = match *index == self.lower_bound {
                false => Some(ValidEntityRange {
                    lower_bound: self.lower_bound,
                    upper_bound: Some(index - 1),
                }),
                true => None,
            };
            self.lower_bound = index + 1;
            new_range
        } else {
            None
        }
    }

    pub(crate) fn merge_with(&mut self, other: &ValidEntityRange) -> bool {
        if self.touches(other) {
            let max_upper_bound: Option<usize> = match (self.upper_bound, other.upper_bound) {
                (None, None) => None,
                (None, Some(_)) | (Some(_), None) => None,
                (Some(a), Some(b)) => Some(a.max(b)),
            };
            let min_lower_bound: usize = self.lower_bound.min(other.lower_bound);

            self.lower_bound = min_lower_bound;
            self.upper_bound = max_upper_bound;

            true
        } else {
            false
        }
    }

    pub(crate) fn contains(&self, index: &usize) -> bool {
        if *index >= self.lower_bound {
            if let Some(upper_bound) = self.upper_bound {
                *index <= upper_bound
            } else {
                true
            }
        } else {
            false
        }
    }

    pub(crate) fn intersects(&self, other: &ValidEntityRange) -> bool {
        let min_upper_bound: Option<usize> = match (self.upper_bound, other.upper_bound) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(a.min(b)),
        };
        let max_lower_bound: usize = self.lower_bound.max(other.lower_bound);

        if let Some(upper_bound) = min_upper_bound {
            max_lower_bound <= upper_bound
        } else {
            true
        }
    }

    pub(crate) fn touches(&self, other: &ValidEntityRange) -> bool {
        let min_upper_bound: Option<usize> = match (self.upper_bound, other.upper_bound) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(a.min(b)),
        };
        let max_lower_bound: usize = self.lower_bound.max(other.lower_bound);

        if let Some(upper_bound) = min_upper_bound {
            max_lower_bound <= upper_bound || max_lower_bound.abs_diff(upper_bound) <= 1
        } else {
            true
        }
    }

    pub(crate) fn is_valid(&self) -> bool {
        if let Some(upper_bound) = self.upper_bound {
            self.lower_bound <= upper_bound
        } else {
            true
        }
    }
}
