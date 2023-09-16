use shallot_dedup::dedup_call;

macro_rules! make_enum {
    ($($variant:ident<$g:tt>,)*) => {
        enum Test {
          $($variant($g)),*
        }
    }
}

dedup_call!(make_enum!, (One<u32> , Two<usize> , Three<usize> , One<u32>, Three<usize>));
