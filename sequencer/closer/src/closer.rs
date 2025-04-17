#![allow(dead_code)]
use l1api::L1Interface;
use l2api::L2Interface;

pub struct Closer<L1, L2> {
    l1: L1,
    l2: L2,
}

impl<L1, L2> Closer<L1, L2>
where
    L1: L1Interface,
    L2: L2Interface,
{
    pub fn new(l1: L1, l2: L2) -> Self {
        Closer { l1, l2 }
    }
}
