pub enum PlaneCut<T> {
    Sibling(T),
    Cut {
        front: Vec<T>,
        back: Vec<T>,
    },
}

pub trait Plane: Sized + Clone {
    fn cut(&self, Self) -> PlaneCut<Self>;
    fn is_aligned(&self, &Self) -> bool;
}

fn add_side<I>(side: &mut Option<Box<BspNode<I::Item>>>, mut iter: I)
where I: Iterator, I::Item: Plane {
    match *side {
        None => {
            if let Some(p) = iter.next() {
                let mut node = BspNode::new(p);
                for p in iter {
                    node.insert(p)
                }
                *side = Some(Box::new(node));
            }
        }
        Some(ref mut node) => {
            for p in iter {
                node.insert(p)
            }
        }
    }
}

pub struct BspNode<T> {
    values: Vec<T>,
    front: Option<Box<BspNode<T>>>,
    back: Option<Box<BspNode<T>>>,
}

impl<T: Plane> BspNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            values: vec![value],
            front: None,
            back: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.front.is_none() && self.back.is_none()
    }

    pub fn insert(&mut self, value: T) {
        match self.values[0].cut(value) {
            PlaneCut::Sibling(value) => self.values.push(value),
            PlaneCut::Cut { mut front, mut back } => {
                add_side(&mut self.front, front.drain(..));
                add_side(&mut self.back, back.drain(..));
            }
        }
    }

    pub fn order(&self, base: &T, out: &mut Vec<T>) {
        let (former, latter) = if base.is_aligned(&self.values[0]) {
            (&self.front, &self.back)
        } else {
            (&self.back, &self.front)
        };

        if let Some(ref node) = *former {
            node.order(base, out);
        }

        out.extend_from_slice(&self.values);

        if let Some(ref node) = *latter {
            node.order(base, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct Plane1D(i32, bool);

    impl Plane for Plane1D {
        fn cut(&self, plane: Self) -> PlaneCut<Self> {
            if plane.0 == self.0 {
                PlaneCut::Sibling(plane)
            } else if self.is_aligned(&plane) {
                PlaneCut::Cut {
                    front: vec![plane],
                    back: vec![],
                }
            } else {
                PlaneCut::Cut {
                    front: vec![],
                    back: vec![plane],
                }
            }
        }

        fn is_aligned(&self, plane: &Self) -> bool {
            (self.0 >= plane.0) == self.1
        }
    }


    #[test]
    fn test_add_side() {
        let mut node_opt = None;
        let p0: Vec<Plane1D> = Vec::new();
        add_side(&mut node_opt, p0.into_iter());
        assert!(node_opt.is_none());

        let p1 = Plane1D(1, true);
        add_side(&mut node_opt, Some(p1.clone()).into_iter());
        assert_eq!(node_opt.as_ref().unwrap().values, vec![p1.clone()]);
        assert!(node_opt.as_ref().unwrap().is_leaf());

        let p23 = vec![Plane1D(0, false), Plane1D(2, false)];
        add_side(&mut node_opt, p23.into_iter());
        let node = node_opt.unwrap();
        assert_eq!(node.values, vec![p1.clone()]);
        assert!(node.front.is_some() && node.back.is_some());
    }
}
