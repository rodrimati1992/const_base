#[derive(Copy, Clone)]
pub(crate) struct ByteSet(pub(crate) [bool; 256]);

impl ByteSet {
    pub(crate) fn remove_range(&mut self, range: core::ops::RangeInclusive<u8>) {
        self.0[*range.start() as usize..=*range.end() as usize].fill(false);
    }
    pub(crate) fn remove(&mut self, byte: u8) {
        self.0[byte as usize] = false;
    }
    pub(crate) fn insert(&mut self, byte: u8) {
        self.0[byte as usize] = true;
    }

    pub(crate) fn iter(&self) -> impl DoubleEndedIterator<Item = (u8, bool)> + Clone + '_ {
        self.0
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| (i as u8, x))
    }
}

#[test]
fn byteset_insert_test() {
    let mut set = ByteSet([false; 256]);

    {
        let mut iter = set.iter();
        for i in 0..=255u8 {
            assert_eq!(iter.next(), Some((i, false)));
        }
    }
    {
        let mut iter = set.iter().filter(|x| x.1);
        assert_eq!(iter.next(), None);
    }
    {
        set.insert(b'=');
        set.insert(b' ');
        set.insert(b'4');
        let mut iter = set.iter().filter(|x| x.1);
        assert_eq!(iter.next(), Some((b' ', true)));
        assert_eq!(iter.next(), Some((b'4', true)));
        assert_eq!(iter.next(), Some((b'=', true)));
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn byteset_remove_test() {
    let mut set = ByteSet([true; 256]);
    {
        let mut iter = set.iter();
        for i in 0..=255u8 {
            assert_eq!(iter.next(), Some((i, true)));
        }
    }
    set.remove_range(0..=b'0');
    set.remove_range(b'6'..=255);
    set.remove(b'4');
    {
        let mut iter = set.iter().filter(|x| x.1);
        assert_eq!(iter.next(), Some((b'1', true)));
        assert_eq!(iter.next(), Some((b'2', true)));
        assert_eq!(iter.next(), Some((b'3', true)));
        assert_eq!(iter.next(), Some((b'5', true)));
        assert_eq!(iter.next(), None);
    }
}
