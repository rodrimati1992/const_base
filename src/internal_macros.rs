macro_rules! for_range_inc {
    ($pat:pat in $start:expr , $end:expr => $($code:tt)*) => ({
        let mut start = $start;
        let end = $end;

        while start <= end {
            let $pat = start;

            $($code)*

            start+=1;
        }
    });
}

macro_rules! for_range {
    ($pat:pat in $range:expr => $($code:tt)*) => {
        let core::ops::Range{mut start, end} = $range;

        while start < end {
            let $pat = start;

            $($code)*

            start+=1;
        }
    };
}

macro_rules! write_into {
    ($array:ident, $index:ident, $b:expr) => {{
        $array[$index] = $b;
        #[allow(unused_assignments)]
        {
            $index += 1;
        }
    }};
}
